mod app;
mod indexer;
mod tui;

use anyhow::Result;

fn main() -> Result<()> {
    // Initialize the index if it doesn't exist
    if !std::path::Path::new("emoji_index").exists() {
        indexer::create_index()?;
    }

    // Run the TUI
    tui::run()?;
    Ok(())
}
