use serde::Deserialize;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Emoji {
    pub emoji: String,
    pub name: String,
    pub short_name: String,
    pub category: String,
}

impl From<&EmojiEntry> for Emoji {
    fn from(value: &EmojiEntry) -> Self {
        let emoji = value
            .unified
            .split('-')
            .filter_map(|hex| u32::from_str_radix(hex, 16).ok())
            .filter_map(char::from_u32)
            .collect::<String>();

        Emoji {
            emoji,
            name: value.name.clone(),
            short_name: value.short_name.clone(),
            category: value.category.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct EmojiEntry {
    pub name: String,
    pub unified: String,
    pub short_name: String,
    pub short_names: Vec<String>,
    pub category: String,
    pub subcategory: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub texts: Vec<String>,
}

pub trait SearcherBackend {
    type Error;

    fn search(&mut self, query: &str) -> Result<Vec<Emoji>, Self::Error>;
}

pub trait IndexerBackend {
    type Error;

    fn index<'a, I>(&mut self, emojis: I) -> Result<(), Self::Error>
    where
        I: Iterator<Item = EmojiEntry>;
}

pub enum Backend {
    Tantivy(crate::tantivy::Backend),
    Nucleo(crate::nucleo::Backend),
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("tantivy")]
    Tantivy(#[source] crate::tantivy::IndexError),
    #[error("nucleo")]
    Nucleo(#[source] crate::nucleo::IndexError),
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[error("tantivy")]
    Tantivy(#[source] crate::tantivy::SearchError),
    #[error("nucleo")]
    Nucleo(#[source] crate::nucleo::SearchError),
}

impl Backend {
    pub fn index<'a, I>(&mut self, emojis: I) -> Result<(), IndexError>
    where
        I: Iterator<Item = EmojiEntry>,
    {
        match self {
            Backend::Tantivy(backend) => backend.index(emojis).map_err(IndexError::Tantivy),
            Backend::Nucleo(backend) => backend.index(emojis).map_err(IndexError::Nucleo),
        }
    }

    pub fn search(&mut self, query: &str) -> Result<Vec<Emoji>, SearchError> {
        match self {
            Backend::Tantivy(backend) => backend.search(query).map_err(SearchError::Tantivy),
            Backend::Nucleo(backend) => backend.search(query).map_err(SearchError::Nucleo),
        }
    }
}
