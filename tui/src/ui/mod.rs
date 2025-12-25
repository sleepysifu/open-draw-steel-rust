mod abilities;
mod combat;
mod entities;
mod setup;
mod status;

use odsr_engine::{BattleParameters,  TurnSide};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{app::{App, BattleMode}, ui::entities::{render_all_entities_setup}};

pub fn render_ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("ODSR Battle System")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content area
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Left side: Battle state or creation
    let left_content = match app.state {
        Some(BattleMode::Active(ref state)) => combat::render_combat_state(state),
        Some(BattleMode::Setup(ref params)) => setup::render_creation_ui(app, params),
        None => {
            let empty_params = BattleParameters::new(Vec::<String>::new(), Vec::<String>::new(), TurnSide::PC);
            setup::render_creation_ui(app, &empty_params)
        },
    };
    f.render_widget(left_content, main_chunks[0]);

    // Right side: Available entities, all entities (for removal), abilities (during turn), or instructions
    let right_content = match (&app.state, &app.input_mode) {
        (Some(BattleMode::Active(state)), crate::app::InputMode::RemovingEntity) => {
            entities::render_all_entities(state)
        }
        (Some(BattleMode::Active(state)), _) => {
            // If a turn is in progress, show abilities; otherwise show available entities
            if state.current_turn().is_some() {
                abilities::render_abilities(state)
            } else {
                entities::render_available_entities(state)
            }
        }
        (Some(BattleMode::Setup(params)), crate::app::InputMode::RemovingEntity) => {
            render_all_entities_setup(params)
        }
        (Some(BattleMode::Setup(_)), _) => setup::render_instructions_setup(),
        _ => combat::render_instructions_combat(),
    };
    f.render_widget(right_content, main_chunks[1]);

    // Message/Status bar or text input
    let status_widget = status::render_status_widget(app);
    f.render_widget(status_widget, chunks[2]);
}


