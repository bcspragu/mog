use std::sync::Arc;

use nucleo::{Config, Nucleo};
use thiserror::Error;

use crate::indexer::{Emoji, EmojiEntry, IndexerBackend, SearcherBackend};

pub struct Backend {
    prev_results: Vec<Emoji>,
    prev_query: String,
    nucleo: Nucleo<EmojiEntry>,
}

#[derive(Error, Debug)]
pub enum IndexError {}
#[derive(Error, Debug)]
pub enum SearchError {}

impl Backend {
    pub fn new() -> Self {
        Backend {
            prev_results: vec![],
            prev_query: String::new(),
            nucleo: Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 5),
        }
    }
}

impl IndexerBackend for Backend {
    type Error = IndexError;

    fn index<'a, I>(&mut self, emojis: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = crate::indexer::EmojiEntry>,
    {
        let injector = self.nucleo.injector();
        for emoji in emojis {
            injector.push(emoji, |emoji, cols| {
                cols[0] = emoji.name.clone().into();
                cols[1] = emoji.unified.clone().into();
                cols[2] = emoji.short_name.clone().into();
                cols[3] = emoji.category.clone().into();
                cols[4] = emoji.subcategory.clone().into();
            });
        }
        Ok(())
    }
}

impl SearcherBackend for Backend {
    type Error = SearchError;

    fn search(&mut self, query: &str) -> Result<Vec<crate::indexer::Emoji>, Self::Error> {
        let append = query.starts_with(&self.prev_query);
        self.prev_query = query.to_string();
        for i in 0..5usize {
            self.nucleo.pattern.reparse(
                i,
                query,
                nucleo::pattern::CaseMatching::Ignore,
                nucleo::pattern::Normalization::Smart,
                append,
            );
        }
        let status = loop {
            let status = self.nucleo.tick(100);
            if !status.running {
                break status;
            }
        };
        if !status.changed {
            return Ok(self.prev_results.clone());
        }
        let snapshot = self.nucleo.snapshot();
        self.prev_results = snapshot.matched_items(..).map(|v| v.data.into()).collect();

        Ok(self.prev_results.clone())
    }
}
