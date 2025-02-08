use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, BoostQuery, FuzzyTermQuery, Occur, RegexQuery};
use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{doc, Index, TantivyDocument, TantivyError, Term};
use thiserror::Error;

use crate::indexer::{Emoji, EmojiEntry, IndexerBackend, SearcherBackend};

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("creating index")]
    IndexCreation(#[source] tantivy::TantivyError),
    #[error("writing handle to index")]
    IndexWriter(#[source] tantivy::TantivyError),
    #[error("adding document to index")]
    AddDoc(#[source] tantivy::TantivyError),
    #[error("committing updated index")]
    Commit(#[source] tantivy::TantivyError),
}

pub struct Backend {
    index: Index,
}

impl Backend {
    pub fn new() -> Result<Self, TantivyError> {
        let index = Index::open_in_dir("emoji_index")?;

        Ok(Self { index })
    }
}

impl IndexerBackend for Backend {
    type Error = IndexError;

    fn index<'a, I>(&mut self, emojis: I) -> std::result::Result<(), Self::Error>
    where
        I: Iterator<Item = EmojiEntry>,
    {
        if std::path::Path::new("emoji_index/meta.json").exists() {
            return Ok(());
        }

        // Define schema
        let mut schema_builder = Schema::builder();
        let emoji_field = schema_builder.add_text_field("emoji", TEXT | STORED);
        let name_field = schema_builder.add_text_field("name", TEXT | STORED);
        let short_name_field = schema_builder.add_text_field("short_name", TEXT | STORED);
        let category_field = schema_builder.add_text_field("category", TEXT | STORED);
        let schema = schema_builder.build();

        // Create index
        let index =
            Index::create_in_dir("emoji_index", schema).map_err(IndexError::IndexCreation)?;
        let mut index_writer = index.writer(50_000_000).map_err(IndexError::IndexWriter)?;

        // Index emoji data
        for entry in emojis {
            let emoji = entry
                .unified
                .split('-')
                .filter_map(|hex| u32::from_str_radix(hex, 16).ok())
                .filter_map(char::from_u32)
                .collect::<String>();

            // Create searchable description combining various fields
            let description = format!(
                "{} {} {} {}",
                entry.name,
                entry.short_names.join(" "),
                entry.category,
                entry.subcategory
            );

            index_writer
                .add_document(doc!(
                    emoji_field => emoji,
                    name_field => entry.name.clone(),
                    short_name_field => entry.short_name.clone(),
                    category_field => description,
                ))
                .map_err(IndexError::AddDoc)?;
        }

        index_writer.commit().map_err(IndexError::Commit)?;
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("getting read handle to index")]
    IndexReader(#[source] tantivy::TantivyError),
    #[error("substring regex was invalid")]
    InvalidRegexPattern(#[source] tantivy::TantivyError),
    #[error("executing search")]
    SearchFailed(#[source] tantivy::TantivyError),
    #[error("retrieving doc")]
    RetrievingDoc(#[source] tantivy::TantivyError),
}

impl SearcherBackend for Backend {
    type Error = SearchError;

    fn search(&mut self, query: &str) -> Result<Vec<crate::indexer::Emoji>, Self::Error> {
        let reader = self.index.reader().map_err(SearchError::IndexReader)?;
        let searcher = reader.searcher();

        let emoji_field = self.index.schema().get_field("emoji").unwrap();
        let name_field = self.index.schema().get_field("name").unwrap();
        let short_name_field = self.index.schema().get_field("short_name").unwrap();
        let category_field = self.index.schema().get_field("category").unwrap();

        let term = Term::from_field_text(name_field, query);
        let fuzzy_query = Box::new(FuzzyTermQuery::new(term, 2, true));
        let regex_query = Box::new(
            RegexQuery::from_pattern(&format!(".*{}.*", query), name_field)
                .map_err(SearchError::InvalidRegexPattern)?,
        );
        let query = BooleanQuery::new(vec![
            (Occur::Should, fuzzy_query),
            (Occur::Should, Box::new(BoostQuery::new(regex_query, 3.0))),
        ]);
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(50))
            .map_err(SearchError::SearchFailed)?;

        let mut emojis = vec![];
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher
                .doc::<TantivyDocument>(doc_address)
                .map_err(SearchError::RetrievingDoc)?;
            let emoji = retrieved_doc
                .get_first(emoji_field)
                .and_then(|f| match f {
                    tantivy::schema::OwnedValue::Str(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("")
                .to_string();
            let name = retrieved_doc
                .get_first(name_field)
                .and_then(|f| match f {
                    tantivy::schema::OwnedValue::Str(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("")
                .to_string();
            let short_name = retrieved_doc
                .get_first(short_name_field)
                .and_then(|f| match f {
                    tantivy::schema::OwnedValue::Str(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("")
                .to_string();
            let category = retrieved_doc
                .get_first(category_field)
                .and_then(|f| match f {
                    tantivy::schema::OwnedValue::Str(s) => Some(s.as_str()),
                    _ => None,
                })
                .unwrap_or("")
                .to_string();
            emojis.push(Emoji {
                emoji,
                name,
                short_name,
                category,
            });
        }

        Ok(emojis)
    }
}
