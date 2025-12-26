use odsr_engine::CombatParameters;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crate::app::App;

pub fn render_creation_ui(_app: &App, params: &CombatParameters) -> Paragraph<'static> {
    let mut text = vec![
        Line::from(Span::styled(
            "Create a new Combat",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    
    text.push(Line::from(Span::styled(
        "PCs in combat:",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));
    let pcs = params.pcs();
    if pcs.is_empty() {
        text.push(Line::from(Span::styled(
            "  (none)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let pc_vec: Vec<&String> = pcs.iter().collect();
        
        for pc in pc_vec {
            text.push(Line::from(Span::styled(
                format!("  • {}", pc),
                Style::default().fg(Color::White),
            )));
        }
    }
    
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "NPCs in combat:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));
    let npcs = params.npcs();
    if npcs.is_empty() {
        text.push(Line::from(Span::styled(
            "  (none)",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        let npc_vec: Vec<&String> = npcs.iter().collect();
        for npc in npc_vec {
            text.push(Line::from(Span::styled(
                format!("  • {}", npc),
                Style::default().fg(Color::White),
            )));
        }
    }
    
    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Combat Setup"))
        .wrap(Wrap { trim: true })
}

pub fn render_instructions_setup() -> Paragraph<'static> {
    let text = vec![
        Line::from(Span::styled(
            "Setup Instructions",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("• Press 'p' to add a PC"),
        Line::from("• Press 'b' to add an NPC"),
        Line::from("• Press 'x' to remove an PC or NPC"),
        Line::from("• Press 'n' to start combat"),
        Line::from(""),
        Line::from("• Press 'q' to quit"),
    ];

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(Wrap { trim: true })
}

pub fn render_monster_definitions(app: &App) -> Paragraph<'static> {
    let mut items: Vec<Line> = vec![Line::from(Span::styled(
        "Select monster definition:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    ))];
    
    items.push(Line::from(""));
    
    if app.monster_definitions.is_empty() {
        items.push(Line::from(Span::styled(
            "No monster definitions available",
            Style::default().fg(Color::Red),
        )));
    } else {
        let definitions: Vec<&String> = app.monster_definitions.keys().collect();
        for (idx, def_name) in definitions.iter().enumerate() {
            let number = idx + 1; // Display 1-based numbers
            let definition = app.monster_definitions.get(*def_name).unwrap();
            items.push(Line::from(vec![
                Span::styled(
                    format!("[{}] ", number),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{} (max stamina: {})", def_name, definition.max_stamina),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "Press 'x' to cancel",
        Style::default().fg(Color::Yellow),
    )));
    
    Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Monster Definitions"))
        .wrap(Wrap { trim: true })
}

pub fn render_hero_definitions(app: &App) -> Paragraph<'static> {
    let mut items: Vec<Line> = vec![Line::from(Span::styled(
        "Select hero definition:",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    ))];
    
    items.push(Line::from(""));
    
    if app.hero_definitions.is_empty() {
        items.push(Line::from(Span::styled(
            "No hero definitions available",
            Style::default().fg(Color::Red),
        )));
    } else {
        let definitions: Vec<&String> = app.hero_definitions.keys().collect();
        for (idx, def_name) in definitions.iter().enumerate() {
            let number = idx + 1; // Display 1-based numbers
            let definition = app.hero_definitions.get(*def_name).unwrap();
            items.push(Line::from(vec![
                Span::styled(
                    format!("[{}] ", number),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{} (max stamina: {})", def_name, definition.max_stamina),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
    }
    
    items.push(Line::from(""));
    items.push(Line::from(Span::styled(
        "Press 'x' to cancel",
        Style::default().fg(Color::Yellow),
    )));
    
    Paragraph::new(items)
        .block(Block::default().borders(Borders::ALL).title("Hero Definitions"))
        .wrap(Wrap { trim: true })
}