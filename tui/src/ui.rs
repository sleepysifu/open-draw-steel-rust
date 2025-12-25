use odsr_engine::{BattleParameters, BattleState, TurnSide};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use crate::app::{App, BattleMode, TextInputType};

pub fn ui(f: &mut Frame, app: &App) {
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
        Some(BattleMode::Active(ref state)) => render_battle_state(state),
        Some(BattleMode::Setup(ref params)) => render_creation_ui(app, params),
        None => {
            let empty_params = BattleParameters::new(Vec::<String>::new(), Vec::<String>::new(), TurnSide::PC);
            render_creation_ui(app, &empty_params)
        },
    };
    f.render_widget(left_content, main_chunks[0]);

    // Right side: Available entities or instructions
    let right_content = match app.state {
        Some(BattleMode::Active(ref state)) => render_available_entities(state),
        _ => render_instructions(),
    };
    f.render_widget(right_content, main_chunks[1]);

    // Message/Status bar or text input
    let status_widget = if let Some(ref text_input) = app.text_input {
        let prompt = match text_input.input_type {
            TextInputType::NPCName => "NPC Name: ",
            TextInputType::PCName => "PC Name: ",
        };
        let input_text = format!("{}{}_", prompt, text_input.buffer);
        Paragraph::new(input_text)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Input"))
    } else {
        Paragraph::new(app.message.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Status"))
    };
    f.render_widget(status_widget, chunks[2]);
}

pub fn render_battle_state(state: &BattleState) -> Paragraph<'static> {
    let current_side = state.current_side();
    let round = state.round();
    let all_pcs = state.all_pcs();
    let all_npcs = state.all_npcs();
    let pc_taken = state.pc_taken_turns();
    let npc_taken = state.npc_taken_turns();
    let current_turn = state.current_turn();

    let mut text = vec![
        Line::from(vec![
            Span::styled("Current Side: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{:?}", current_side),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Round: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{}", round),
                Style::default().fg(Color::Cyan),
            ),
        ]),
    ];

    // Show current turn in progress
    if let Some((side, name)) = current_turn {
        text.push(Line::from(vec![
            Span::styled("Turn in progress: ", Style::default().fg(Color::White)),
            Span::styled(
                format!("{} ({:?})", name, side),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]));
    }
    else {
        text.push(Line::from(Span::styled(
            "Select a character to start their turn",
            Style::default().fg(Color::White),
        )));
    }

    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "PCs:",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));

    // Display all PCs with their status
    let mut pc_vec: Vec<&String> = all_pcs.iter().collect();
    pc_vec.sort();
    for pc in pc_vec {
        let style = if let Some((TurnSide::PC, name)) = current_turn {
            if name == pc {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if pc_taken.contains(pc) {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            }
        } else if pc_taken.contains(pc) {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };
        text.push(Line::from(
            Span::styled(format!("  • {}", pc), style),
        ));
    }

    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "NPCs:",
        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
    )));

    // Display all NPCs with their status
    let mut npc_vec: Vec<&String> = all_npcs.iter().collect();
    npc_vec.sort();
    for npc in npc_vec {
        let style = if let Some((TurnSide::NPC, name)) = current_turn {
            if name == npc {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else if npc_taken.contains(npc) {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            }
        } else if npc_taken.contains(npc) {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::White)
        };
        text.push(Line::from(
            Span::styled(format!("  • {}", npc), style),
        ));
    }

    text.push(Line::from(""));
    if current_turn.is_some() {
        text.push(Line::from(Span::styled(
            "Press 'e' to end the current turn",
            Style::default().fg(Color::Yellow),
        )));
        text.push(Line::from(Span::styled(
            "Press 'c' to cancel the current turn",
            Style::default().fg(Color::Yellow),
        )));
    } else {
        text.push(Line::from(Span::styled(
            "Press a number (1-9) to start a turn for that entity",
            Style::default().fg(Color::Gray),
        )));
    }
    text.push(Line::from(Span::styled(
        "Press 'r' to complete the round",
        Style::default().fg(Color::Gray),
    )));

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Battle State"))
        .wrap(Wrap { trim: true })
}

pub fn render_available_entities(state: &BattleState) -> Paragraph<'static> {
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

pub fn render_creation_ui(_app: &App, params: &BattleParameters) -> Paragraph<'static> {
    let mut text = vec![
        Line::from(Span::styled(
            "Create a Battle",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];
    
    text.push(Line::from(Span::styled(
        "PCs in battle:",
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
        "NPCs in battle:",
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
    
    text.push(Line::from(""));
    text.push(Line::from(Span::styled(
        "Press 'p' to add a PC",
        Style::default().fg(Color::Yellow),
    )));
    text.push(Line::from(Span::styled(
        "Press 'b' to add an NPC",
        Style::default().fg(Color::Yellow),
    )));
    text.push(Line::from(Span::styled(
        "Press 'n' to start battle",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )));

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Battle Setup"))
        .wrap(Wrap { trim: true })
}

pub fn render_instructions() -> Paragraph<'static> {
    let text = vec![
        Line::from(Span::styled(
            "Instructions",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("• Press a number (1-9) to start a turn"),
        Line::from("• Press 'e' to end the current turn"),
        Line::from("• Press 'c' to cancel the current turn"),
        Line::from("• Press 'r' to complete round"),
        Line::from("• Press 'q' to quit"),
    ];

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(Wrap { trim: true })
}

