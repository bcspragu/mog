mod app;
mod indexer;
mod nucleo;
mod tantivy;
mod tui;

use anyhow::{Context, Result};
use indexer::{Backend, EmojiEntry};

fn main() -> Result<()> {
    let search_backend = std::env::var("SEARCH_BACKEND").unwrap_or("nucleo".to_string());

    let mut backend = match search_backend.as_str() {
        "tantivy" => Backend::Tantivy(crate::tantivy::Backend::new()?),
        "nucleo" => Backend::Nucleo(crate::nucleo::Backend::new()),
        v => panic!("unknown backend {}", v),
    };

    // Read and parse emoji data
    let file = std::fs::File::open("emoji-slim.json").context("reading emoji database")?;
    let emojis: Vec<EmojiEntry> =
        serde_json::from_reader(file).context("deserializing emoji database")?;

    backend.index(emojis.into_iter())?;

    // Run the TUI
    tui::run(backend)?;
    Ok(())
}
