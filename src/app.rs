use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::Index;

pub struct App {
    pub input: String,
    pub emojis: Vec<(String, String)>, // (emoji, description)
    pub selected: usize,
    index: Index,
    query_parser: QueryParser,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let index = Index::open_in_dir("emoji_index")?;
        let query_parser = QueryParser::for_index(&index, vec![
            index.schema().get_field("name").unwrap(),
            index.schema().get_field("short_name").unwrap(),
            index.schema().get_field("category").unwrap(),
        ]);
        
        Ok(Self {
            input: String::new(),
            emojis: Vec::new(),
            selected: 0,
            index,
            query_parser,
        })
    }

    pub fn search(&mut self) -> anyhow::Result<()> {
        if self.input.is_empty() {
            self.emojis.clear();
            return Ok(());
        }

        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        
        let query = self.query_parser.parse_query(&self.input)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(50))?;

        self.emojis.clear();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: tantivy::Document = searcher.doc(doc_address)?;
            let emoji = retrieved_doc.get_first(self.index.schema().get_field("emoji").unwrap())
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            let description = retrieved_doc.get_first(self.index.schema().get_field("description").unwrap())
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();
            self.emojis.push((emoji, description));
        }

        Ok(())
    }

    pub fn on_key(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn backspace(&mut self) {
        self.input.pop();
    }

    pub fn move_selection(&mut self, delta: i32) {
        let new_selected = self.selected as i32 + delta;
        self.selected = new_selected.clamp(0, self.emojis.len().saturating_sub(1) as i32) as usize;
    }

    pub fn selected_emoji(&self) -> Option<&str> {
        self.emojis.get(self.selected).map(|(emoji, _)| emoji.as_str())
    }
}
