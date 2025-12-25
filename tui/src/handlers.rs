use crossterm::event::KeyCode;
use odsr_engine::{CombatParameters, CombatState, TurnSide};
use odsr_engine::dice::rolld10s;
use crate::app::{App, CombatMode, InputMode, TextInput, TextInputType};

pub fn handle_creation_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('n') => {
            create_combat(app);
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
        KeyCode::Char('x') => {
            // Enter removal mode during setup
            if let Some(CombatMode::Setup(_)) = app.state {
                app.input_mode = InputMode::RemovingEntity;
                app.message = "Select entity to remove (press number, or 'x' to cancel):".to_string();
            }
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
                    // Cancel input if empty or only whitespace
                    app.text_input = None;
                    // Return to appropriate mode based on current state
                    app.input_mode = match app.state {
                        Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                        Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                        None => InputMode::CreatingCombat,
                    };
                    app.message = "Input cancelled".to_string();
                    return false;
                }
                
                match app.state {
                    Some(CombatMode::Setup(ref mut params)) => {
                        // Adding during setup
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
                        app.input_mode = InputMode::CreatingCombat;
                    }
                    Some(CombatMode::Active(ref state)) => {
                        // Adding/removing during combat
                        match text_input.input_type {
                            TextInputType::NPCName => {
                                match state.add_npc(name.clone()) {
                                    Ok(new_state) => {
                                        app.state = Some(CombatMode::Active(new_state));
                                        app.message = format!("Added NPC: {} (reinforcement)", name);
                                    }
                                    Err(e) => {
                                        app.message = format!("Error: {}", e);
                                    }
                                }
                            }
                            TextInputType::PCName => {
                                match state.add_pc(name.clone()) {
                                    Ok(new_state) => {
                                        app.state = Some(CombatMode::Active(new_state));
                                        app.message = format!("Added PC: {} (reinforcement)", name);
                                    }
                                    Err(e) => {
                                        app.message = format!("Error: {}", e);
                                    }
                                }
                            }
                        }
                        app.input_mode = InputMode::TakingTurn;
                    }
                    None => {
                        app.message = "No combat state available".to_string();
                        app.input_mode = InputMode::CreatingCombat;
                    }
                }
                
                // Exit text input mode
                app.text_input = None;
            }
            KeyCode::Esc => {
                // Cancel text input
                app.text_input = None;
                // Return to appropriate mode based on current state
                app.input_mode = match app.state {
                    Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                    Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                    None => InputMode::CreatingCombat,
                };
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
        KeyCode::Char('b') => {
            // Enter text input mode for NPC name (during combat)
            if let Some(CombatMode::Active(_)) = app.state {
                app.input_mode = InputMode::TextInput;
                app.text_input = Some(TextInput {
                    buffer: String::new(),
                    input_type: TextInputType::NPCName,
                });
                app.message = "Enter NPC name to add (press Enter to confirm, Esc to cancel):".to_string();
            }
        }
        KeyCode::Char('p') => {
            // Enter text input mode for PC name (during combat)
            if let Some(CombatMode::Active(_)) = app.state {
                app.input_mode = InputMode::TextInput;
                app.text_input = Some(TextInput {
                    buffer: String::new(),
                    input_type: TextInputType::PCName,
                });
                app.message = "Enter PC name to add (press Enter to confirm, Esc to cancel):".to_string();
            }
        }
        KeyCode::Char('x') => {
            // Enter removal mode
            if let Some(CombatMode::Active(_)) = app.state {
                app.input_mode = InputMode::RemovingEntity;
                app.message = "Select entity to remove (press number, or 'x' to cancel):".to_string();
            }
        }
        KeyCode::Char('r') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                let new_state = state.complete_round();
                app.state = match new_state {
                    Ok(new_state) => Some(CombatMode::Active(new_state)),
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                        return false;
                    }
                };
                app.message = "Round completed!".to_string();
            }
        }
        KeyCode::Char('e') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                match state.end_turn() {
                    Ok(new_state) => {
                        app.state = Some(CombatMode::Active(new_state));
                        app.message = "Turn ended".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char('c') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                match state.cancel_turn() {
                    Ok(new_state) => {
                        app.state = Some(CombatMode::Active(new_state));
                        app.message = "Turn cancelled".to_string();
                    }
                    Err(e) => {
                        app.message = format!("Error: {}", e);
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            if let Some(CombatMode::Active(ref state)) = app.state {
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
                                app.state = Some(CombatMode::Active(new_state));
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

pub fn create_combat(app: &mut App) {
    let combat_params = match &app.state {
        Some(CombatMode::Setup(params)) => params,
        _ => {
            app.message = "combat setup not found!".to_string();
            return;
        }
    };
    
    if combat_params.pcs().is_empty() || combat_params.npcs().is_empty() {
        app.message = "Please add at least one PC and one NPC first!".to_string();
        return;
    }

    let starting_roll:i32 = rolld10s(1).iter().sum();
    let starting_side = if starting_roll > 5 {
        TurnSide::PC
    } else {
        TurnSide::NPC
    };

    // Create new CombatParameters with updated starting side
    let combat_parameters = CombatParameters::new(
        combat_params.pcs().clone(),
        combat_params.npcs().clone(),
        starting_side,
    );

    app.state = Some(CombatMode::Active(CombatState::new(combat_parameters)));
    app.input_mode = InputMode::TakingTurn;
    app.message = format!(
        "combat created! Starting side: {:?} (rolled {})",
        starting_side, starting_roll
    );
}

pub fn handle_removal_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('x') => {
            // Cancel removal mode - return to appropriate mode
            app.input_mode = match app.state {
                Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                None => InputMode::CreatingCombat,
            };
            app.message = "Removal cancelled".to_string();
        }
        KeyCode::Char(c) => {
            // Check if it's a digit (1-9)
            if let Some(digit) = c.to_digit(10) {
                match app.state {
                    Some(CombatMode::Setup(ref mut params)) => {
                        // Removal during setup
                        let all_pcs: Vec<String> = params.pcs().iter().cloned().collect();
                        let all_npcs: Vec<String> = params.npcs().iter().cloned().collect();
                        let mut all_entities: Vec<(String, bool)> = Vec::new(); // (name, is_pc)
                        
                        for pc in all_pcs {
                            all_entities.push((pc, true));
                        }
                        for npc in all_npcs {
                            all_entities.push((npc, false));
                        }
                        
                        let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                        
                        if index < all_entities.len() {
                            let (entity_name, is_pc) = &all_entities[index];
                            let removed = if *is_pc {
                                params.remove_pc(entity_name)
                            } else {
                                params.remove_npc(entity_name)
                            };
                            
                            if removed {
                                let entity_type = if *is_pc { "PC" } else { "NPC" };
                                app.message = format!("Removed {}: {}", entity_type, entity_name);
                                app.input_mode = InputMode::CreatingCombat;
                            } else {
                                app.message = format!("Entity '{}' not found", entity_name);
                            }
                        } else {
                            app.message = format!("No entity at position {}", digit);
                        }
                    }
                    Some(CombatMode::Active(ref state)) => {
                        // Removal during combat
                        let all_pcs: Vec<String> = state.all_pcs().iter().cloned().collect();
                        let all_npcs: Vec<String> = state.all_npcs().iter().cloned().collect();
                        let mut all_entities: Vec<(String, bool)> = Vec::new(); // (name, is_pc)
                        
                        for pc in all_pcs {
                            all_entities.push((pc, true));
                        }
                        for npc in all_npcs {
                            all_entities.push((npc, false));
                        }
                        
                        let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                        
                        if index < all_entities.len() {
                            let (entity_name, is_pc) = &all_entities[index];
                            let result = if *is_pc {
                                state.remove_pc(entity_name)
                            } else {
                                state.remove_npc(entity_name)
                            };
                            
                            match result {
                                Ok(new_state) => {
                                    app.state = Some(CombatMode::Active(new_state));
                                    let entity_type = if *is_pc { "PC" } else { "NPC" };
                                    app.message = format!("Removed {}: {} (death)", entity_type, entity_name);
                                    app.input_mode = InputMode::TakingTurn;
                                }
                                Err(e) => {
                                    app.message = format!("Error: {}", e);
                                }
                            }
                        } else {
                            app.message = format!("No entity at position {}", digit);
                        }
                    }
                    None => {
                        app.message = "No combat state available".to_string();
                    }
                }
            }
        }
        _ => {}
    }
    false
}

