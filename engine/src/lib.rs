pub mod combat;
pub mod dice;
pub mod npc;
pub mod pc;

pub use combat::{CombatParameters, CombatState, TurnSide};
pub use dice::{rolld3s, rolld10s, power_roll};
pub use npc::NPC;
pub use pc::PC;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{combat::{CombatParameters, CombatState, TurnSide}, dice::rolld10s, npc::NPC, pc::PC};

    #[test]
    fn test_combat_flow() {
        let mut pcs: HashSet<PC> = HashSet::new();
        pcs.insert(PC::new("PC1".to_string()));
        pcs.insert(PC::new("PC2".to_string()));
        pcs.insert(PC::new("PC3".to_string()));
    
        let mut npcs: HashSet<NPC> = HashSet::new();
        npcs.insert(NPC::new("NPC1".to_string()));
        npcs.insert(NPC::new("NPC2".to_string()));
        npcs.insert(NPC::new("NPC3".to_string()));
    
        let starting_roll:i32 = rolld10s(1).iter().sum();
        let starting_side = if starting_roll > 5 {
            TurnSide::PC
        } else {
            TurnSide::NPC
        };
    
        let combat_parameters = CombatParameters::new(
            pcs.iter().map(|pc| pc.name().clone()),
            npcs.iter().map(|npc| npc.name().clone()),
            starting_side,
        );
        
        let combat = CombatState::new(combat_parameters);
        println!("combat started: {:?}", combat);
        
        // Start PC1's turn
        let combat = match combat.start_turn(TurnSide::PC, "PC1".to_string()) {
            Ok(new_state) => {
                println!("PC1 started their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                combat
            }
        };
        println!("combat state after starting PC1 turn: {:?}", combat);
        
        // End PC1's turn
        let combat = match combat.end_turn() {
            Ok(new_state) => {
                println!("PC1 ended their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                combat
            }
        };
        println!("combat state after ending PC1 turn: {:?}", combat);
        
        // Start NPC1's turn
        let combat = match combat.start_turn(TurnSide::NPC, "NPC1".to_string()) {
            Ok(new_state) => {
                println!("NPC1 started their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                combat
            }
        };
        println!("combat state after starting NPC1 turn: {:?}", combat);
        
        // End NPC1's turn
        let combat = match combat.end_turn() {
            Ok(new_state) => {
                println!("NPC1 ended their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                combat
            }
        };
        println!("combat state after ending NPC1 turn: {:?}", combat);
    }
}