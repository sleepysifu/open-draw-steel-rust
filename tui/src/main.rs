use std::collections::HashSet;
use std::io::{self, stdout};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use odsr_engine::{BattleParameters, BattleState, NPC, PC, TurnSide};
use odsr_engine::dice::rolld10s;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

enum BattleMode {
    Setup(BattleParameters),
    Active(BattleState),
}

struct App {
    battle: Option<BattleMode>,
    pcs: HashSet<PC>,
    npcs: HashSet<NPC>,
    message: String,
    input_mode: InputMode,
}

enum InputMode {
    CreatingBattle,
    TakingTurn,
}

impl Default for App {
    fn default() -> App {
        let mut pcs = HashSet::new();
        pcs.insert(PC::new("PC1".to_string()));
        pcs.insert(PC::new("PC2".to_string()));
        pcs.insert(PC::new("PC3".to_string()));
        
        let mut npcs = HashSet::new();
        npcs.insert(NPC::new("NPC1".to_string()));
        npcs.insert(NPC::new("NPC2".to_string()));
        npcs.insert(NPC::new("NPC3".to_string()));
        
        let battle_params = BattleParameters::new(
            HashSet::new(),
            HashSet::new(),
            TurnSide::PC,
        );
        
        App {
            battle: Some(BattleMode::Setup(battle_params)),
            pcs,
            npcs,
            message: "Welcome! Press 'n' to start the battle, or 'q' to quit.".to_string(),
            input_mode: InputMode::CreatingBattle,
        }
    }
}

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
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::CreatingBattle => {
                        should_quit = handle_creation_input(&mut app, key.code);
                    }
                    InputMode::TakingTurn => {
                        should_quit = handle_turn_input(&mut app, key.code);
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

fn handle_creation_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('n') => {
            create_battle(app);
        }
        KeyCode::Char('a') => {
            app.message = "Enter PC name (then press Enter):".to_string();
        }
        KeyCode::Char('b') => {
            if let Some(BattleMode::Setup(ref mut params)) = app.battle {
                // Find first NPC not already in battle
                let npc_to_add = app.npcs.iter()
                    .find(|npc| !params.npcs().contains(npc.name()));
                
                if let Some(npc) = npc_to_add {
                    let npc_name = npc.name().clone();
                    params.add_npc(npc_name.clone());
                    app.message = format!("Added NPC: {}", npc_name);
                } else {
                    app.message = "All NPCs already added or no NPCs available".to_string();
                }
            }
        }
        KeyCode::Char('p') => {
            if let Some(BattleMode::Setup(ref mut params)) = app.battle {
                // Find first PC not already in battle
                let pc_to_add = app.pcs.iter()
                    .find(|pc| !params.pcs().contains(pc.name()));
                
                if let Some(pc) = pc_to_add {
                    let pc_name = pc.name().clone();
                    params.add_pc(pc_name.clone());
                    app.message = format!("Added PC: {}", pc_name);
                } else {
                    app.message = "All PCs already added or no PCs available".to_string();
                }
            }
        }
        _ => {}
    }
    false
}

fn handle_turn_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('r') => {
            if let Some(BattleMode::Active(ref state)) = app.battle {
                let new_state = state.complete_round();
                app.battle = match new_state {
                    Ok(new_state) => Some(BattleMode::Active(new_state)),
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                        return false;
                    }
                };
                app.message = "Round completed!".to_string();
            }
        }
        KeyCode::Char('e') => {
            if let Some(BattleMode::Active(ref state)) = app.battle {
                match state.end_turn() {
                    Ok(new_state) => {
                        app.battle = Some(BattleMode::Active(new_state));
                        app.message = "Turn ended".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char('c') => {
            if let Some(BattleMode::Active(ref state)) = app.battle {
                match state.cancel_turn() {
                    Ok(new_state) => {
                        app.battle = Some(BattleMode::Active(new_state));
                        app.message = "Turn cancelled".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            if let Some(BattleMode::Active(ref state)) = app.battle {
                // Check if it's a digit (1-9)
                if let Some(digit) = c.to_digit(10) {
                    let available: Vec<String> = state.available().into_iter().collect();
                    let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                    
                    if index < available.len() {
                        let entity = &available[index];
                        let side = state.current_side();
                        match state.start_turn(side, entity.clone()) {
                            Ok(new_state) => {
                                app.battle = Some(BattleMode::Active(new_state));
                                app.message = format!("{} started their turn", entity);
                            }
                            Err(e) => {
                                app.message = format!("Error: {}", e);
                            }
                        }
                    } else {
                        app.message = format!("No entity at position {}", digit);
                    }
                }
            }
        }
        _ => {}
    }
    false
}

fn create_battle(app: &mut App) {
    let battle_params = match &app.battle {
        Some(BattleMode::Setup(params)) => params,
        _ => {
            app.message = "Battle setup not found!".to_string();
            return;
        }
    };
    
    if battle_params.pcs().is_empty() || battle_params.npcs().is_empty() {
        app.message = "Please add at least one PC and one NPC first!".to_string();
        return;
    }

    let starting_roll:i32 = rolld10s(1).iter().sum();
    let starting_side = if starting_roll > 5 {
        TurnSide::PC
    } else {
        TurnSide::NPC
    };

    // Create new BattleParameters with updated starting side
    let battle_parameters = BattleParameters::new(
        battle_params.pcs().clone(),
        battle_params.npcs().clone(),
        starting_side,
    );

    app.battle = Some(BattleMode::Active(BattleState::new(battle_parameters)));
    app.input_mode = InputMode::TakingTurn;
    app.message = format!(
        "Battle created! Starting side: {:?} (rolled {})",
        starting_side, starting_roll
    );
}

fn ui(f: &mut Frame, app: &App) {
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
    let left_content = match app.battle {
        Some(BattleMode::Active(ref state)) => render_battle_state(state),
        Some(BattleMode::Setup(ref params)) => render_creation_ui(app, params),
        None => {
            let empty_params = BattleParameters::new(HashSet::new(), HashSet::new(), TurnSide::PC);
            render_creation_ui(app, &empty_params)
        },
    };
    f.render_widget(left_content, main_chunks[0]);

    // Right side: Available entities or instructions
    let right_content = match app.battle {
        Some(BattleMode::Active(ref state)) => render_available_entities(state),
        _ => render_instructions(),
    };
    f.render_widget(right_content, main_chunks[1]);

    // Message/Status bar
    let message = Paragraph::new(app.message.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(message, chunks[2]);
}

fn render_battle_state(state: &BattleState) -> Paragraph<'static> {
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

fn render_available_entities(state: &BattleState) -> Paragraph<'static> {
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

fn render_creation_ui(_app: &App, params: &BattleParameters) -> Paragraph<'static> {
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
        let mut pc_vec: Vec<&String> = pcs.iter().collect();
        pc_vec.sort();
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
        let mut npc_vec: Vec<&String> = npcs.iter().collect();
        npc_vec.sort();
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

fn render_instructions() -> Paragraph<'static> {
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

