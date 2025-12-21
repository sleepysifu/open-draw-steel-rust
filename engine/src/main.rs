mod combat;
mod dice;
mod npc;
mod pc;


fn main() {

   
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{combat::{BattleParameters, BattleState, TurnSide}, dice::roll, npc::NPC, pc::PC};

    #[test]
    fn test_battle_flow() {
        let mut pcs: HashSet<PC> = HashSet::new();
        pcs.insert(PC::new("PC1".to_string()));
        pcs.insert(PC::new("PC2".to_string()));
        pcs.insert(PC::new("PC3".to_string()));
    
        let mut npcs: HashSet<NPC> = HashSet::new();
        npcs.insert(NPC::new("NPC1".to_string()));
        npcs.insert(NPC::new("NPC2".to_string()));
        npcs.insert(NPC::new("NPC3".to_string()));
    
        let starting_roll = roll(0,1,0);
        let starting_side = if starting_roll > 5 {
            TurnSide::PC
        } else {
            TurnSide::NPC
        };
    
        let battle_parameters = BattleParameters::new(
            pcs.iter().map(|pc| pc.name().clone()).collect(),
            npcs.iter().map(|npc| npc.name().clone()).collect(),
            starting_side,
        );
        
        let battle = BattleState::new(battle_parameters);
        println!("Battle started: {:?}", battle);
        
        // Start PC1's turn
        let battle = match battle.start_turn(TurnSide::PC, "PC1".to_string()) {
            Ok(new_state) => {
                println!("PC1 started their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                battle
            }
        };
        println!("Battle state after starting PC1 turn: {:?}", battle);
        
        // End PC1's turn
        let battle = match battle.end_turn() {
            Ok(new_state) => {
                println!("PC1 ended their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                battle
            }
        };
        println!("Battle state after ending PC1 turn: {:?}", battle);
        
        // Start NPC1's turn
        let battle = match battle.start_turn(TurnSide::NPC, "NPC1".to_string()) {
            Ok(new_state) => {
                println!("NPC1 started their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                battle
            }
        };
        println!("Battle state after starting NPC1 turn: {:?}", battle);
        
        // End NPC1's turn
        let battle = match battle.end_turn() {
            Ok(new_state) => {
                println!("NPC1 ended their turn");
                new_state
            }
            Err(e) => {
                println!("Error: {}", e);
                battle
            }
        };
        println!("Battle state after ending NPC1 turn: {:?}", battle);
    }
}