mod combat;
mod dice;
mod npc;
mod pc;

use std::collections::HashSet;
use crate::{combat::{BattleParameters, BattleState, TurnSide}, dice::roll, npc::NPC, pc::PC};
fn main() {

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
    
    let battle = match battle.take_turn(TurnSide::PC, "PC1".to_string()) {
        Ok(new_state) => {
            println!("PC1 took their turn");
            new_state
        }
        Err(e) => {
            println!("Error: {}", e);
            battle
        }
    };
    println!("Battle state after PC turn: {:?}", battle);
    
    let battle = match battle.take_turn(TurnSide::NPC, "NPC1".to_string()) {
        Ok(new_state) => {
            println!("NPC1 took their turn");
            new_state
        }
        Err(e) => {
            println!("Error: {}", e);
            battle
        }
    };
    println!("Battle state after NPC turn: {:?}", battle);
}
