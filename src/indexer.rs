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
    #[cfg(feature = "nucleo")]
    Nucleo(crate::nucleo::Backend),
    #[cfg(feature = "tantivy")]
    Tantivy(crate::tantivy::Backend),
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[cfg(feature = "nucleo")]
    #[error("nucleo")]
    Nucleo(#[source] crate::nucleo::IndexError),

    #[cfg(feature = "tantivy")]
    #[error("tantivy")]
    Tantivy(#[source] crate::tantivy::IndexError),
}

#[derive(Error, Debug)]
pub enum SearchError {
    #[cfg(feature = "nucleo")]
    #[error("nucleo")]
    Nucleo(#[source] crate::nucleo::SearchError),

    #[cfg(feature = "tantivy")]
    #[error("tantivy")]
    Tantivy(#[source] crate::tantivy::SearchError),
}

impl Backend {
    pub fn index<'a, I>(&mut self, emojis: I) -> Result<(), IndexError>
    where
        I: Iterator<Item = EmojiEntry>,
    {
        match self {
            #[cfg(feature = "nucleo")]
            Backend::Nucleo(backend) => backend.index(emojis).map_err(IndexError::Nucleo),

            #[cfg(feature = "tantivy")]
            Backend::Tantivy(backend) => backend.index(emojis).map_err(IndexError::Tantivy),
        }
    }

    pub fn search(&mut self, query: &str) -> Result<Vec<Emoji>, SearchError> {
        match self {
            #[cfg(feature = "nucleo")]
            Backend::Nucleo(backend) => backend.search(query).map_err(SearchError::Nucleo),
            #[cfg(feature = "tantivy")]
            Backend::Tantivy(backend) => backend.search(query).map_err(SearchError::Tantivy),
        }
    }
}
