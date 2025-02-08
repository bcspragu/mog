use crate::indexer::Backend;
use crate::{app::App, indexer::SearchError};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::{self, stdout};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TuiError {
    #[error("enabling raw mode")]
    EnableRawMode(#[source] io::Error),
    #[error("disabling raw mode")]
    DisableRawMode(#[source] io::Error),
    #[error("initializing backend")]
    InitBackend(#[source] io::Error),
    #[error("drawing app")]
    Draw(#[source] io::Error),
    #[error("reading event")]
    ReadEvent(#[source] io::Error),
    #[error("searching emojis")]
    Search(#[from] SearchError),
    #[error("showing cursor")]
    ShowCursor(#[source] io::Error),
}

pub fn run(search_backend: Backend) -> Result<(), TuiError> {
    // Setup terminal
    enable_raw_mode().map_err(TuiError::EnableRawMode)?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(TuiError::InitBackend)?;

    // Create app state
    let mut app = App::new(search_backend);

    // Main loop
    let emoji = loop {
        terminal.draw(|f| ui(f, &app)).map_err(TuiError::Draw)?;

        if let Event::Key(key) = event::read().map_err(TuiError::ReadEvent)? {
            match key.code {
                KeyCode::Char(c) => {
                    app.on_key(c);
                    app.search()?;
                }
                KeyCode::Backspace => {
                    app.backspace();
                    app.search()?;
                }
                KeyCode::Esc => break None,
                KeyCode::Up => app.move_selection(-1),
                KeyCode::Down => app.move_selection(1),
                KeyCode::Enter => {
                    if let Some(emoji) = app.selected_emoji() {
                        break Some(emoji);
                    }
                }
                _ => {}
            }
        }
    };

    // Restore terminal
    disable_raw_mode().map_err(TuiError::DisableRawMode)?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen);
    terminal.show_cursor().map_err(TuiError::ShowCursor)?;

    if let Some(emoji) = emoji {
        print!("{}", emoji);
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.area());

    // Search input
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(input, chunks[0]);

    // Emoji list
    let items: Vec<ListItem> = app
        .emojis
        .iter()
        .map(|e| ListItem::new(format!("{} - {}", e.emoji, e.name)))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Emojis").borders(Borders::ALL))
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        list,
        chunks[1],
        &mut ListState::default().with_selected(Some(app.selected)),
    );
}
