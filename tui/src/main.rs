mod app;
mod handlers;
mod ui;

use std::{io::{self, Stdout, stdout}};
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
use handlers::{handle_creation_input, handle_turn_input, handle_text_input, handle_removal_input, handle_monster_selection, handle_hero_selection, handle_ability_selection, handle_target_selection};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    match run(&mut terminal) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
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

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), String> {
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            return Err(e);
        }
    };
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| ui::render_ui(f, &app)).map_err(|e| e.to_string())?;

        if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
            if key.kind == KeyEventKind::Press {
                // Handle log view toggle (works in any mode)
                if let crossterm::event::KeyCode::Char('`') = key.code {
                    app.log_view_expanded = !app.log_view_expanded;
                    continue;
                }
                
                match app.input_mode {
                    InputMode::CreatingCombat => {
                        should_quit = handle_creation_input(&mut app, key.code);
                    }
                    InputMode::TakingTurn => {
                        should_quit = handle_turn_input(&mut app, key.code);
                    }
                    InputMode::TextInput(_) => {
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
                    InputMode::SelectingAbility => {
                        should_quit = handle_ability_selection(&mut app, key.code);
                    }
                    InputMode::SelectingTarget { .. } => {
                        should_quit = handle_target_selection(&mut app, key.code);
                    }
                }
            }
        }
    }
    Ok(())
}