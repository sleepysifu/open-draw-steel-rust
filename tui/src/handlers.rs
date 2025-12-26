use crossterm::event::KeyCode;
use odsr_engine::{CombatParameters, CombatState, TurnSide, Entity};
use odsr_engine::dice::rolld10s;
use crate::app::{App, CombatMode, InputMode, TextInput, TextInputType};

/// Counts how many entities exist with the given definition name
fn count_instances_of_definition(app: &App, definition_name: &String) -> usize {
    app.entities
        .values()
        .filter(|entity| entity.definition_name() == definition_name)
        .count()
}

/// Generates a default name for an entity based on definition name and instance count
fn generate_default_entity_name(definition_name: &String, instance_count: usize) -> String {
    format!("{} {}", definition_name, instance_count + 1)
}

/// Ensures an entity instance exists with the given instance name, using the specified definition.
/// Requires definition_name to be provided - no ad-hoc definitions are created.
/// Creates the entity instance if it doesn't exist.
fn ensure_entity_exists(app: &mut App, instance_name: &String, definition_name: &String, is_hero: bool) {
    // Get the definition from the appropriate map
    let definition = if is_hero {
        app.definitions.heroes.get(definition_name)
            .expect("Hero definition must exist")
            .clone()
    } else {
        app.definitions.monsters.get(definition_name)
            .expect("Monster definition must exist")
            .clone()
    };
    
    // Create entity instance if it doesn't exist
    if !app.entities.contains_key(instance_name) {
        let entity = Entity::new(instance_name.clone(), definition);
        app.entities.insert(instance_name.clone(), entity);
    }
}

pub fn handle_creation_input(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('n') => {
            create_combat(app);
        }
        KeyCode::Char('b') => {
            // Enter monster definition selection mode for NPC
            if app.definitions.monsters.is_empty() {
                app.log("No monster definitions available. Add some to content/monsters/ first.".to_string());
            } else {
                app.input_mode = InputMode::SelectingMonsterDefinition;
                app.log("Select monster definition (press number, or 'x' to cancel):".to_string());
            }
        }
        KeyCode::Char('p') => {
            // Enter hero definition selection mode for PC
            if app.definitions.heroes.is_empty() {
                app.log("No hero definitions available. Add some to content/heroes/ first.".to_string());
            } else {
                app.input_mode = InputMode::SelectingHeroDefinition;
                app.log("Select hero definition (press number, or 'x' to cancel):".to_string());
            }
        }
        KeyCode::Char('x') => {
            // Enter removal mode during setup
            if let Some(CombatMode::Setup(_)) = app.state {
                app.input_mode = InputMode::RemovingEntity;
                app.log("Select entity to remove (press number, or 'x' to cancel):".to_string());
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
                let input_type = text_input.input_type;
                
                if name.is_empty() {
                    // Cancel input if empty or only whitespace
                    app.text_input = None;
                    // Return to appropriate mode based on current state
                    app.input_mode = match app.state {
                        Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                        Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                        None => InputMode::CreatingCombat,
                    };
                    app.log("Input cancelled".to_string());
                    return false;
                }
                
                // Extract selected definition before borrowing app mutably
                let selected_def = text_input.selected_definition.clone()
                    .expect("Definition must be selected");
                
                // Ensure entity exists before checking combat state
                match input_type {
                    TextInputType::NPCName => {
                        ensure_entity_exists(app, &name, &selected_def, false); // false = monster
                    }
                    TextInputType::PCName => {
                        ensure_entity_exists(app, &name, &selected_def, true); // true = hero
                    }
                }

                match app.state {
                    Some(CombatMode::Setup(ref mut params)) => {
                        // Adding during setup
                        match input_type {
                            TextInputType::NPCName => {
                                if params.npcs().contains(&name) {
                                    app.log(format!("NPC '{}' already added", name));
                                } else {
                                    params.add_npc(name.clone());
                                    app.log(format!("Added NPC: {}", name));
                                }
                            }
                            TextInputType::PCName => {
                                if params.pcs().contains(&name) {
                                    app.log(format!("PC '{}' already added", name));
                                } else {
                                    params.add_pc(name.clone());
                                    app.log(format!("Added PC: {}", name));
                                }
                            }
                        }
                        app.input_mode = InputMode::CreatingCombat;
                    }
                    Some(CombatMode::Active(ref state)) => {
                        // Adding/removing during combat
                        match input_type {
                            TextInputType::NPCName => {
                                match state.add_npc(name.clone()) {
                                    Ok(new_state) => {
                                        app.state = Some(CombatMode::Active(new_state));
                                        app.log(format!("Added NPC: {} (reinforcement)", name));
                                    }
                                    Err(e) => {
                                        app.log(format!("Error: {}", e));
                                    }
                                }
                            }
                            TextInputType::PCName => {
                                match state.add_pc(name.clone()) {
                                    Ok(new_state) => {
                                        app.state = Some(CombatMode::Active(new_state));
                                        app.log(format!("Added PC: {} (reinforcement)", name));
                                    }
                                    Err(e) => {
                                        app.log(format!("Error: {}", e));
                                    }
                                }
                            }
                        }
                        app.input_mode = InputMode::TakingTurn;
                    }
                    None => {
                        app.log("No combat state available".to_string());
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
                app.log("Input cancelled".to_string());
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
            // Enter monster definition selection mode for NPC (during combat)
            if let Some(CombatMode::Active(_)) = app.state {
                if app.definitions.monsters.is_empty() {
                    app.log("No monster definitions available. Add some to content/monsters/ first.".to_string());
                } else {
                    app.input_mode = InputMode::SelectingMonsterDefinition;
                    app.log("Select monster definition (press number, or 'x' to cancel):".to_string());
                }
            }
        }
            KeyCode::Char('p') => {
            // Enter hero definition selection mode for PC (during combat)
            if let Some(CombatMode::Active(_)) = app.state {
                if app.definitions.heroes.is_empty() {
                    app.log("No hero definitions available. Add some to content/heroes/ first.".to_string());
                } else {
                    app.input_mode = InputMode::SelectingHeroDefinition;
                    app.log("Select hero definition (press number, or 'x' to cancel):".to_string());
                }
            }
        }
        KeyCode::Char('x') => {
            // Enter removal mode
            if let Some(CombatMode::Active(_)) = app.state {
                    app.input_mode = InputMode::RemovingEntity;
                    app.log("Select entity to remove (press number, or 'x' to cancel):".to_string());
            }
        }
        KeyCode::Char('r') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                let new_state = state.complete_round();
                app.state = match new_state {
                    Ok(new_state) => Some(CombatMode::Active(new_state)),
                    Err(e) => {
                        app.log(format!("Error: {}", e));
                        return false;
                    }
                };
                app.log("Round completed!".to_string());
            }
        }
        KeyCode::Char('e') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                match state.end_turn() {
                    Ok(new_state) => {
                        app.state = Some(CombatMode::Active(new_state));
                        app.selected_ability = None; // Clear selected ability
                        app.log("Turn ended".to_string());
                    }
                    Err(e) => {
                        app.log(format!("Error: {}", e));
                    }
                }
            }
        }
        KeyCode::Char('a') => {
            // Enter ability selection mode
            if let Some(CombatMode::Active(ref state)) = app.state {
                if let Some((_side, entity_name)) = state.current_turn() {
                    if let Some(entity) = app.entities.get(entity_name) {
                        let ability_names = &entity.definition().abilities;
                        if ability_names.is_empty() {
                            app.log("No abilities available for this entity.".to_string());
                        } else {
                            app.input_mode = InputMode::SelectingAbility;
                            app.log("Select ability (press number, or 'x' to cancel):".to_string());
                        }
                    }
                }
            }
        }
        KeyCode::Char('c') => {
            if let Some(CombatMode::Active(ref state)) = app.state {
                match state.cancel_turn() {
                    Ok(new_state) => {
                        app.state = Some(CombatMode::Active(new_state));
                        app.log("Turn cancelled".to_string());
                    }
                    Err(e) => {
                        app.log(format!("Error: {}", e));
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
                                app.log(format!("{} started their turn", entity));
                            }
                            Err(e) => {
                                app.log(format!("Error: {}", e));
                            }
                        }
                    } else {
                        app.log(format!("No entity at position {}", digit));
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
            app.log("combat setup not found!".to_string());
            return;
        }
    };
    
    if combat_params.pcs().is_empty() || combat_params.npcs().is_empty() {
        app.log("Please add at least one PC and one NPC first!".to_string());
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
    app.log(format!(
        "combat created! Starting side: {:?} (rolled {})",
        starting_side, starting_roll
    ));
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
            app.log("Removal cancelled".to_string());
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
                                app.log(format!("Removed {}: {}", entity_type, entity_name));
                                app.input_mode = InputMode::CreatingCombat;
                            } else {
                                app.log(format!("Entity '{}' not found", entity_name));
                            }
                        } else {
                            app.log(format!("No entity at position {}", digit));
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
                                    app.log(format!("Removed {}: {} (death)", entity_type, entity_name));
                                    app.input_mode = InputMode::TakingTurn;
                                }
                                Err(e) => {
                                    app.log(format!("Error: {}", e));
                                }
                            }
                        } else {
                            app.log(format!("No entity at position {}", digit));
                        }
                    }
                    None => {
                        app.log("No combat state available".to_string());
                    }
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_monster_selection(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('x') => {
            // Cancel selection - return to creation mode
            app.input_mode = match app.state {
                Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                None => InputMode::CreatingCombat,
            };
            app.log("Monster selection cancelled".to_string());
        }
        KeyCode::Char(c) => {
            // Check if it's a digit (1-9)
            if let Some(digit) = c.to_digit(10) {
                let definitions: Vec<&String> = app.definitions.monsters.keys().collect();
                let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                
                if index < definitions.len() {
                    let definition_name = definitions[index].clone();
                    let instance_count = count_instances_of_definition(app, &definition_name);
                    let default_name = generate_default_entity_name(&definition_name, instance_count);
                    
                    // Enter text input mode with pre-filled name
                    app.input_mode = InputMode::TextInput;
                    app.text_input = Some(TextInput {
                        buffer: default_name.clone(),
                        input_type: TextInputType::NPCName,
                        selected_definition: Some(definition_name),
                    });
                    app.log("Enter NPC name (press Enter to confirm, Esc to cancel):".to_string());
                } else {
                    app.log(format!("No monster definition at position {}", digit));
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_hero_selection(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('x') => {
            // Cancel selection - return to creation mode
            app.input_mode = match app.state {
                Some(CombatMode::Setup(_)) => InputMode::CreatingCombat,
                Some(CombatMode::Active(_)) => InputMode::TakingTurn,
                None => InputMode::CreatingCombat,
            };
            app.log("Hero selection cancelled".to_string());
        }
        KeyCode::Char(c) => {
            // Check if it's a digit (1-9)
            if let Some(digit) = c.to_digit(10) {
                let definitions: Vec<&String> = app.definitions.heroes.keys().collect();
                let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                
                if index < definitions.len() {
                    let definition_name = definitions[index].clone();
                    let instance_count = count_instances_of_definition(app, &definition_name);
                    let default_name = generate_default_entity_name(&definition_name, instance_count);
                    
                    // Enter text input mode with pre-filled name
                    app.input_mode = InputMode::TextInput;
                    app.text_input = Some(TextInput {
                        buffer: default_name.clone(),
                        input_type: TextInputType::PCName,
                        selected_definition: Some(definition_name),
                    });
                    app.log("Enter PC name (press Enter to confirm, Esc to cancel):".to_string());
                } else {
                    app.log(format!("No hero definition at position {}", digit));
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_ability_selection(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('x') => {
            // Cancel ability selection - return to turn mode
            app.input_mode = InputMode::TakingTurn;
            app.selected_ability = None;
            app.log("Ability selection cancelled".to_string());
        }
        KeyCode::Char(c) => {
            // Check if it's a digit (1-9)
            if let Some(digit) = c.to_digit(10) {
                if let Some(CombatMode::Active(ref state)) = app.state {
                    if let Some((_side, entity_name)) = state.current_turn() {
                        if let Some(entity) = app.entities.get(entity_name) {
                            let ability_names: Vec<&String> = entity.definition().abilities.iter().collect();
                            let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                            
                            if index < ability_names.len() {
                                let ability_name = ability_names[index].clone();
                                // Verify ability exists
                                if app.definitions.abilities.contains_key(&ability_name) {
                                    app.selected_ability = Some(ability_name.clone());
                                    app.input_mode = InputMode::SelectingTarget;
                                    app.log(format!("Selected ability: {}. Select target (press number, or 'x' to cancel):", ability_name));
                                } else {
                                    app.log(format!("Ability '{}' not found in definitions", ability_name));
                                }
                            } else {
                                app.log(format!("No ability at position {}", digit));
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    false
}

pub fn handle_target_selection(app: &mut App, key: KeyCode) -> bool {
    match key {
        KeyCode::Char('q') => return true,
        KeyCode::Char('x') => {
            // Cancel target selection - return to ability selection
            app.input_mode = InputMode::SelectingAbility;
            app.log("Target selection cancelled. Select ability (press number, or 'x' to cancel):".to_string());
        }
        KeyCode::Char(c) => {
            // Check if it's a digit (1-9)
            if let Some(digit) = c.to_digit(10) {
                if let Some(CombatMode::Active(ref state)) = app.state {
                    // Get all entities in combat (PCs and NPCs)
                    let all_pcs: Vec<&String> = state.all_pcs().iter().collect();
                    let all_npcs: Vec<&String> = state.all_npcs().iter().collect();
                    let mut all_entities: Vec<&String> = Vec::new();
                    
                    for pc in all_pcs {
                        all_entities.push(pc);
                    }
                    for npc in all_npcs {
                        all_entities.push(npc);
                    }
                    
                    let index = (digit as usize).saturating_sub(1); // Convert 1-9 to 0-8
                    
                    if index < all_entities.len() {
                        let target_name = all_entities[index].clone();
                        
                        // Execute ability (stub)
                        if let Some(ability_name) = app.selected_ability.clone() {
                            execute_ability(app, &ability_name, &target_name);
                        }
                    } else {
                        app.log(format!("No entity at position {}", digit));
                    }
                }
            }
        }
        _ => {}
    }
    false
}

fn execute_ability(app: &mut App, ability_name: &str, target_name: &str) {
    // Stub: Just log the execution
    app.log(format!("Executing ability '{}' on target '{}' (stub)", ability_name, target_name));
    
    // After execution, return to turn mode
    app.input_mode = InputMode::TakingTurn;
    app.selected_ability = None;
    app.log("Ability executed. Press 'e' to end turn, or 'a' to use another ability.".to_string());
}
