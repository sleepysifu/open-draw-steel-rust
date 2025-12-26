mod app;
mod handlers;
mod ui;

use std::io::{self, stdout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use app::{App, InputMode};
use handlers::{handle_creation_input, handle_turn_input, handle_text_input, handle_removal_input, handle_monster_selection, handle_hero_selection};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| ui::render_ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::CreatingCombat => {
                        should_quit = handle_creation_input(&mut app, key.code);
                    }
                    InputMode::TakingTurn => {
                        should_quit = handle_turn_input(&mut app, key.code);
                    }
                    InputMode::TextInput => {
                        should_quit = handle_text_input(&mut app, key.code);
                    }
                    InputMode::RemovingEntity => {
                        should_quit = handle_removal_input(&mut app, key.code);
                    }
                    InputMode::SelectingMonsterDefinition => {
                        should_quit = handle_monster_selection(&mut app, key.code);
                    }
                    InputMode::SelectingHeroDefinition => {
                        should_quit = handle_hero_selection(&mut app, key.code);
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
