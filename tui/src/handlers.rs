use crossterm::event::KeyCode;
use odsr_engine::{BattleParameters, BattleState, TurnSide};
use odsr_engine::dice::rolld10s;
use crate::app::{App, BattleMode, InputMode, TextInput, TextInputType};

pub fn handle_creation_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('n') => {
            create_battle(app);
        }
        KeyCode::Char('b') => {
            // Enter text input mode for NPC name
            app.input_mode = InputMode::TextInput;
            app.text_input = Some(TextInput {
                buffer: String::new(),
                input_type: TextInputType::NPCName,
            });
            app.message = "Enter NPC name (press Enter to confirm, Esc to cancel):".to_string();
        }
        KeyCode::Char('p') => {
            // Enter text input mode for PC name
            app.input_mode = InputMode::TextInput;
            app.text_input = Some(TextInput {
                buffer: String::new(),
                input_type: TextInputType::PCName,
            });
            app.message = "Enter PC name (press Enter to confirm, Esc to cancel):".to_string();
        }
        _ => {}
    }
    false
}

pub fn handle_text_input(app: &mut App, key: KeyCode) -> bool {
    if let Some(ref mut text_input) = app.text_input {
        match key {
            KeyCode::Enter => {
                // Submit the name
                let name = text_input.buffer.trim().to_string();
                if name.is_empty() {
                    app.message = "Name cannot be empty!".to_string();
                    return false;
                }
                
                if let Some(BattleMode::Setup(ref mut params)) = app.state {
                    match text_input.input_type {
                        TextInputType::NPCName => {
                            if params.npcs().contains(&name) {
                                app.message = format!("NPC '{}' already added", name);
                            } else {
                                params.add_npc(name.clone());
                                app.message = format!("Added NPC: {}", name);
                            }
                        }
                        TextInputType::PCName => {
                            if params.pcs().contains(&name) {
                                app.message = format!("PC '{}' already added", name);
                            } else {
                                params.add_pc(name.clone());
                                app.message = format!("Added PC: {}", name);
                            }
                        }
                    }
                }
                
                // Exit text input mode
                app.text_input = None;
                app.input_mode = InputMode::CreatingBattle;
            }
            KeyCode::Esc => {
                // Cancel text input
                app.text_input = None;
                app.input_mode = InputMode::CreatingBattle;
                app.message = "Input cancelled".to_string();
            }
            KeyCode::Backspace => {
                text_input.buffer.pop();
            }
            KeyCode::Char(c) => {
                text_input.buffer.push(c);
            }
            _ => {}
        }
    }
    false
}

pub fn handle_turn_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('r') => {
            if let Some(BattleMode::Active(ref state)) = app.state {
                let new_state = state.complete_round();
                app.state = match new_state {
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
            if let Some(BattleMode::Active(ref state)) = app.state {
                match state.end_turn() {
                    Ok(new_state) => {
                        app.state = Some(BattleMode::Active(new_state));
                        app.message = "Turn ended".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char('c') => {
            if let Some(BattleMode::Active(ref state)) = app.state {
                match state.cancel_turn() {
                    Ok(new_state) => {
                        app.state = Some(BattleMode::Active(new_state));
                        app.message = "Turn cancelled".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            if let Some(BattleMode::Active(ref state)) = app.state {
                // Check if it's a digit (1-9)
                if let Some(digit) = c.to_digit(10) {
                    // IndexSet preserves insertion order, so we can index directly
                    let available: Vec<String> = state.available().into_iter().collect();
                    let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                    
                    if index < available.len() {
                        let entity = &available[index];
                        let side = state.current_side();
                        match state.start_turn(side, entity.clone()) {
                            Ok(new_state) => {
                                app.state = Some(BattleMode::Active(new_state));
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

pub fn create_battle(app: &mut App) {
    let battle_params = match &app.state {
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

    app.state = Some(BattleMode::Active(BattleState::new(battle_parameters)));
    app.input_mode = InputMode::TakingTurn;
    app.message = format!(
        "Battle created! Starting side: {:?} (rolled {})",
        starting_side, starting_roll
    );
}

