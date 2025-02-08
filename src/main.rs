mod app;
mod indexer;
mod nucleo;
mod tantivy;
mod tui;

use indexer::{Backend, EmojiEntry};

fn main() -> Result<(), String> {
    let search_backend = std::env::var("SEARCH_BACKEND").unwrap_or("nucleo".to_string());

    let mut backend = match search_backend.as_str() {
        "tantivy" => Backend::Tantivy(
            crate::tantivy::Backend::new()
                .map_err(|e| format!("failed to init tantivy backend: {:?}", e))?,
        ),
        "nucleo" => Backend::Nucleo(crate::nucleo::Backend::new()),
        v => panic!("unknown backend {}", v),
    };

    // Read and parse emoji data
    let file = std::fs::File::open("emoji-slim.json")
        .map_err(|e| format!("reading emoji database: {:?}", e))?;
    let emojis: Vec<EmojiEntry> = serde_json::from_reader(file)
        .map_err(|e| format!("deserializing emoji database: {:?}", e))?;

    backend
        .index(emojis.into_iter())
        .map_err(|e| format!("indexing emojis: {:?}", e))?;

    // Run the TUI
    tui::run(backend).map_err(|e| format!("running tui {:?}", e))?;
    Ok(())
}
