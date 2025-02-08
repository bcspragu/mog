mod app;
mod indexer;
#[cfg(feature = "nucleo")]
mod nucleo;
#[cfg(feature = "tantivy")]
mod tantivy;
mod tui;

use indexer::{Backend, EmojiEntry};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let initial_input = args.get(1).map(|v| v.to_owned());

    let search_backend = std::env::var("SEARCH_BACKEND").unwrap_or("nucleo".to_string());
    let feeling_lucky = std::env::var("IM_FEELING_LUCKY")
        .map(|v| v.trim() != "")
        .unwrap_or(false);

    if feeling_lucky && initial_input.is_none() {
        return Err("No input was given, but IM_FEELING_LUCKY was specified".to_string());
    }

    let mut backend = match search_backend.as_str() {
        #[cfg(feature = "nucleo")]
        "nucleo" => Backend::Nucleo(crate::nucleo::Backend::new()),
        #[cfg(feature = "tantivy")]
        "tantivy" => Backend::Tantivy(
            crate::tantivy::Backend::new()
                .map_err(|e| format!("failed to init tantivy backend: {:?}", e))?,
        ),
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

    if feeling_lucky {
        // We can unwrap this, we confirmed it was set above.
        let query = initial_input.unwrap();
        let emojis = backend
            .search(&query)
            .map_err(|e| format!("failed to run search: {:?}", e))?;
        print!(
            "{}",
            emojis
                .get(0)
                .map(|e| e.emoji.clone())
                .unwrap_or(format!("No emojis found for '{}'", query))
        )
    } else {
        // Run the TUI
        tui::run(backend, initial_input).map_err(|e| format!("running tui {:?}", e))?;
    }
    Ok(())
}
