use odsr_engine::{CombatState, TurnSide};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_combat_state(state: &CombatState) -> Paragraph<'static> {
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
    

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Battle State"))
        .wrap(Wrap { trim: true })
}

pub fn render_instructions_combat() -> Paragraph<'static> {
    let text = vec![
        Line::from(Span::styled(
            "Battle Instructions",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Turn Management:",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from("• Press a number (1-9) to start a turn"),
        Line::from("• Press 'e' to end the current turn"),
        Line::from("• Press 'c' to cancel the current turn"),
        Line::from("• Press 'r' to complete round"),
        Line::from(""),
        Line::from(Span::styled(
            "Reinforcements/Deaths:",
            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
        )),
        Line::from("• Press 'p' to add a PC"),
        Line::from("• Press 'b' to add an NPC"),
        Line::from("• Press 'x' to remove an PC or NPC"),
        Line::from(""),
        Line::from("• Press 'q' to quit"),
    ];

    Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .wrap(Wrap { trim: true })
}