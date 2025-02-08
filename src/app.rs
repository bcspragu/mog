use crate::indexer::{Backend, Emoji, SearchError};

#[allow(dead_code)]
pub struct App {
    pub input: String,
    pub emojis: Vec<Emoji>,
    pub selected: usize,
    backend: Backend,
}

impl App {
    pub fn new(backend: Backend, initial_input: Option<String>) -> Self {
        Self {
            input: initial_input.unwrap_or(String::new()),
            emojis: Vec::new(),
            selected: 0,
            backend,
        }
    }

    pub fn search(&mut self) -> Result<(), SearchError> {
        if self.input.is_empty() {
            self.emojis.clear();
            return Ok(());
        }

        self.emojis = self.backend.search(&self.input)?;

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
        self.emojis.get(self.selected).map(|e| e.emoji.as_str())
    }
}
