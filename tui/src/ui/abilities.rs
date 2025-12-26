use odsr_engine::CombatState;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

use super::entities::render_available_entities;


pub fn render_abilities(state: &CombatState, app: &App) -> Paragraph<'static> {
    use crate::app::InputMode;
    
    // If selecting ability, show selection UI
    if let InputMode::SelectingAbility = app.input_mode {
        return render_ability_selection(state, app);
    }
    let current_turn = state.current_turn();
    
    if let Some((_side, entity_name)) = current_turn {
        let entity = match app.entities.get(entity_name){
            Some(entity) => entity,
            None => {
                return Paragraph::new(Line::from(format!("Error: Entity {} not found", entity_name)));
            }
        };
        
        let mut text = vec![
            Line::from(Span::styled(
                format!("{}'s Abilities", entity_name),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];
        
        let ability_names = &entity.definition().abilities;
        
        if ability_names.is_empty() {
            text.push(Line::from(Span::styled(
                "No abilities available",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (idx, ability_name) in ability_names.iter().enumerate() {
                let number = idx + 1;
                if let Some(ability) = app.definitions.abilities.get(ability_name) {
                    text.push(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", number),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{}", ability.name),
                            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                        ),
                    ]));
                } else {
                    text.push(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", number),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{} (not found)", ability_name),
                            Style::default().fg(Color::Red),
                        ),
                    ]));
                }
            }
            
            text.push(Line::from(""));
            text.push(Line::from(Span::styled(
                "Press 'a' to use an ability",
                Style::default().fg(Color::Yellow),
            )));
        }
        
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Abilities"))
            .wrap(Wrap { trim: true })
    } else {
        // Fallback (shouldn't happen, but just in case)
        render_available_entities(state)
    }
}

fn render_ability_selection(state: &CombatState, app: &App) -> Paragraph<'static> {
    let current_turn = state.current_turn();
    
    if let Some((_side, entity_name)) = current_turn {
        let entity = match app.entities.get(entity_name){
            Some(entity) => entity,
            None => {
                return Paragraph::new(Line::from(format!("Error: Entity {} not found", entity_name)));
            }
        };
        
        let mut text = vec![
            Line::from(Span::styled(
                format!("Select {}'s Ability", entity_name),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];
        
        let ability_names = &entity.definition().abilities;
        
        if ability_names.is_empty() {
            text.push(Line::from(Span::styled(
                "No abilities available",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for (idx, ability_name) in ability_names.iter().enumerate() {
                let number = idx + 1;
                if let Some(ability) = app.definitions.abilities.get(ability_name) {
                    text.push(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", number),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{}", ability.name),
                            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                        ),
                    ]));
                } else {
                    text.push(Line::from(vec![
                        Span::styled(
                            format!("[{}] ", number),
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{} (not found)", ability_name),
                            Style::default().fg(Color::Red),
                        ),
                    ]));
                }
            }
        }
        
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            "Press 'x' to cancel",
            Style::default().fg(Color::Yellow),
        )));
        
        Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Select Ability"))
            .wrap(Wrap { trim: true })
    } else {
        render_available_entities(state)
    }
}