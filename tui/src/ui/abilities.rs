use odsr_engine::CombatState;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

use super::entities::render_available_entities;


pub fn render_abilities(state: &CombatState, app: &App) -> Paragraph<'static> {
    let current_turn = state.current_turn();
    
    if let Some((_side, entity_name)) = current_turn {
        let entity = match app.entities.get(entity_name){
            Some(entity) => entity,
            None => {
                return Paragraph::new(Line::from(format!("Error: Entity {} not found", entity_name)));
            }
        };
        let _abilities = &entity.definition().abilities;
        let text = vec![
            Line::from(Span::styled(
                format!("{}'s Abilities", entity_name),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "(Abilities will be displayed here)",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Abilities"))
            .wrap(Wrap { trim: true })
    } else {
        // Fallback (shouldn't happen, but just in case)
        render_available_entities(state)
    }
}