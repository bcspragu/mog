use anyhow::Result;
use serde::Deserialize;
use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{doc, Index};

#[derive(Deserialize)]
struct EmojiEntry {
    name: String,
    unified: String,
    short_name: String,
    short_names: Vec<String>,
    category: String,
    subcategory: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    texts: Vec<String>,
}

pub fn create_index() -> Result<()> {
    // Define schema
    let mut schema_builder = Schema::builder();
    let emoji_field = schema_builder.add_text_field("emoji", TEXT | STORED);
    let name_field = schema_builder.add_text_field("name", TEXT | STORED);
    let short_name_field = schema_builder.add_text_field("short_name", TEXT | STORED);
    let category_field = schema_builder.add_text_field("category", TEXT | STORED);
    let schema = schema_builder.build();

    // Create index
    let index = Index::create_in_dir("emoji_index", schema)?;
    let mut index_writer = index.writer(50_000_000)?;

    // Read and parse emoji data
    let file = std::fs::File::open("emoji-slim.json")?;
    let emojis: Vec<EmojiEntry> = serde_json::from_reader(file)?;

    // Index emoji data
    for entry in emojis {
        // Convert unified code to emoji
        let emoji = String::from_utf16_lossy(&[u16::from_str_radix(&entry.unified, 16)?]);
        
        // Create searchable description combining various fields
        let description = format!(
            "{} {} {} {}",
            entry.name,
            entry.short_names.join(" "),
            entry.category,
            entry.subcategory
        );

        index_writer.add_document(doc!(
            emoji_field => emoji,
            name_field => entry.name,
            short_name_field => entry.short_name,
            category_field => description,
        ))?;
    }

    index_writer.commit()?;
    Ok(())
}
