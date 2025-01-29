use crate::app::App;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::io::stdout;

pub fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => {
                    app.on_key(c);
                    app.search()?;
                }
                KeyCode::Backspace => {
                    app.backspace();
                    app.search()?;
                }
                KeyCode::Up => app.move_selection(-1),
                KeyCode::Down => app.move_selection(1),
                KeyCode::Enter => {
                    if let Some(_emoji) = app.selected_emoji() {
                        // Copy to clipboard could be implemented here
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.area());

    // Search input
    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(input, chunks[0]);

    // Emoji list
    let items: Vec<ListItem> = app
        .emojis
        .iter()
        .map(|(emoji, description)| {
            ListItem::new(format!("{} - {}", emoji, description))
        })
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
