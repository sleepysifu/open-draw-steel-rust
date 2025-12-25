use odsr_engine::{BattleParameters, CombatState, TurnSide};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_available_entities(state: &CombatState) -> Paragraph<'static> {
    let available = state.available();
    let current_side = state.current_side();
    
    if available.is_empty() {
        let text = vec![
            Line::from(Span::styled(
                format!("No {:?}s available", current_side),
                Style::default().fg(Color::Red),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press 'r' to complete the round",
                Style::default().fg(Color::Yellow),
            )),
        ];
        
        return Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Available Entities"))
            .wrap(Wrap { trim: true });
    }

    let mut items: Vec<Line> = vec![Line::from(Span::styled(
        format!("Available {:?}s:", current_side),
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    ))];

    // IndexSet preserves insertion order, so we can iterate directly
    for (idx, entity) in available.iter().enumerate() {
        let number = idx + 1; // Display 1-based numbers
        items.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", number),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(entity.clone(), Style::default().fg(Color::White)),
        ]));
    }

    Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Available Entities"))
        .wrap(Wrap { trim: true })
}

pub fn render_all_entities(state: &CombatState) -> Paragraph<'static> {
    let all_pcs = state.all_pcs();
    let all_npcs = state.all_npcs();
    let current_turn = state.current_turn();
    
    let mut items: Vec<Line> = vec![Line::from(Span::styled(
        "Select entity to remove:",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ))];
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "PCs:",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    
    let mut entity_index = 0;
    let pc_vec: Vec<&String> = all_pcs.iter().collect();
    for pc in &pc_vec {
        entity_index += 1;
        let style = if let Some((TurnSide::PC, name)) = current_turn {
            if name == *pc {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            }
        } else {
            Style::default().fg(Color::White)
        };
        items.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", entity_index),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled((*pc).clone(), style),
        ]));
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "NPCs:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));
    
    let npc_vec: Vec<&String> = all_npcs.iter().collect();
    for npc in &npc_vec {
        entity_index += 1;
        let style = if let Some((TurnSide::NPC, name)) = current_turn {
            if name == *npc {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            }
        } else {
            Style::default().fg(Color::White)
        };
        items.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", entity_index),
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
            Span::styled((*npc).clone(), style),
        ]));
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "Press 'x' to cancel",
        Style::default().fg(Color::Yellow),
    )));
    
    Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Remove Entity"))
        .wrap(Wrap { trim: true })
}

pub fn render_all_entities_setup(params: &BattleParameters) -> Paragraph<'static> {
    let all_pcs = params.pcs();
    let all_npcs = params.npcs();
    
    let mut items: Vec<Line> = vec![Line::from(Span::styled(
        "Select entity to remove:",
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    ))];
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "PCs:",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    
    let mut entity_index = 0;
    let pc_vec: Vec<&String> = all_pcs.iter().collect();
    for pc in &pc_vec {
        entity_index += 1;
        items.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", entity_index),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled((*pc).clone(), Style::default().fg(Color::White)),
        ]));
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "NPCs:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));
    
    let npc_vec: Vec<&String> = all_npcs.iter().collect();
    for npc in &npc_vec {
        entity_index += 1;
        items.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", entity_index),
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
            Span::styled((*npc).clone(), Style::default().fg(Color::White)),
        ]));
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "Press 'x' to cancel",
        Style::default().fg(Color::Yellow),
    )));
    
    Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Remove Entity"))
        .wrap(Wrap { trim: true })
}