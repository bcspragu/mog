use anyhow::Result;
use serde::Deserialize;
use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{doc, Index};

#[derive(Deserialize)]
struct EmojiEntry {
    emoji: String,
    description: String,
}

pub fn create_index() -> Result<()> {
    // Define schema
    let mut schema_builder = Schema::builder();
    let emoji_field = schema_builder.add_text_field("emoji", TEXT | STORED);
    let description_field = schema_builder.add_text_field("description", TEXT | STORED);
    let schema = schema_builder.build();

    // Create index
    let index = Index::create_in_dir("emoji_index", schema)?;
    let mut index_writer = index.writer(50_000_000)?;

    // Read and parse emoji data
    let file = std::fs::File::open("emoji-slim.json")?;
    let emojis: Vec<EmojiEntry> = serde_json::from_reader(file)?;

    // Index emoji data
    for entry in emojis {
        index_writer.add_document(doc!(
            emoji_field => entry.emoji,
            description_field => entry.description,
        ))?;
    }

    index_writer.commit()?;
    Ok(())
}
