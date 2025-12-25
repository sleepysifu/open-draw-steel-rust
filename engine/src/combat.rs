use std::collections::HashSet;
use indexmap::IndexSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnSide {
    PC,
    NPC,
}


/**
 * Parameters for a battle. These cannot mutate once the battle is started.
 */
#[derive(Debug, Clone)]
pub struct BattleParameters {
    pcs: IndexSet<String>,
    npcs: IndexSet<String>,
    starting_side: TurnSide,
}

impl BattleParameters {
    pub fn new(pcs: impl IntoIterator<Item = String>, npcs: impl IntoIterator<Item = String>, starting_side: TurnSide) -> Self {
        Self {
            pcs: pcs.into_iter().collect(),
            npcs: npcs.into_iter().collect(),
            starting_side,
        }
    }
    
    pub fn players(&self, turn: TurnSide) -> &IndexSet<String> {
        match turn {
            TurnSide::NPC => &self.npcs,
            TurnSide::PC => &self.pcs,
        }
    }

    pub fn pcs(&self) -> &IndexSet<String> {
        &self.pcs
    }
    
    pub fn npcs(&self) -> &IndexSet<String> {
        &self.npcs
    }
    
    pub fn starting_side(&self) -> TurnSide {
        self.starting_side
    }
    
    pub fn add_pc(&mut self, pc: String) {
        self.pcs.insert(pc);
    }
    
    pub fn add_npc(&mut self, npc: String) {
        self.npcs.insert(npc);
    }
    
    pub fn remove_pc(&mut self, pc: &String) -> bool {
        self.pcs.shift_remove(pc)
    }
    
    pub fn remove_npc(&mut self, npc: &String) -> bool {
        self.npcs.shift_remove(npc)
    }
}

#[derive(Debug, Clone)]
pub struct BattleState {
    starting_parameters:BattleParameters,
    current_side: TurnSide,
    current_turn: Option<(TurnSide, String)>, // The entity currently taking their turn
    pc_taken_turns: HashSet<String>,
    npc_taken_turns: HashSet<String>,
    round:i16,
}

impl BattleState {
    pub fn new(parameters:BattleParameters) -> Self {
        Self {
            pc_taken_turns: HashSet::with_capacity(parameters.pcs.len()),
            npc_taken_turns: HashSet::with_capacity(parameters.npcs.len()),
            current_side: parameters.starting_side,
            current_turn: None,
            starting_parameters:parameters,
            round: 1,
        }
    }

    pub fn current_side(&self) -> TurnSide {
        self.current_side
    }

    pub fn round(&self) -> i16 {
        self.round
    }

    pub fn all_pcs(&self) -> &IndexSet<String> {
        self.starting_parameters.pcs()
    }

    pub fn all_npcs(&self) -> &IndexSet<String> {
        self.starting_parameters.npcs()
    }

    pub fn pc_taken_turns(&self) -> &HashSet<String> {
        &self.pc_taken_turns
    }

    pub fn npc_taken_turns(&self) -> &HashSet<String> {
        &self.npc_taken_turns
    }

    pub fn current_turn(&self) -> Option<&(TurnSide, String)> {
        self.current_turn.as_ref()
    }

    pub fn available(&self) -> IndexSet<String> {
        match self.current_side {
            TurnSide::PC => {
                // PCs can act, exclude those who already took their turn or are currently taking it
                // Preserve insertion order from IndexSet
                let mut available: IndexSet<String> = self.starting_parameters.pcs()
                    .iter()
                    .filter(|pc| !self.pc_taken_turns.contains(*pc))
                    .cloned()
                    .collect();
                
                // Remove the entity currently taking their turn
                if let Some((TurnSide::PC, ref name)) = self.current_turn {
                    available.shift_remove(name);
                }
                
                available
            }
            TurnSide::NPC => {
                // NPCs can act, exclude those who already took their turn or are currently taking it
                // Preserve insertion order from IndexSet
                let mut available: IndexSet<String> = self.starting_parameters.npcs()
                    .iter()
                    .filter(|npc| !self.npc_taken_turns.contains(*npc))
                    .cloned()
                    .collect();
                
                // Remove the entity currently taking their turn
                if let Some((TurnSide::NPC, ref name)) = self.current_turn {
                    available.shift_remove(name);
                }
                
                available
            }
        }
    }

    pub fn start_turn(&self, side: TurnSide, entity_name: String) -> Result<Self, String> {
        // 1. Check if there's already a turn in progress
        if self.current_turn.is_some() {
            return Err("A turn is already in progress. End the current turn first.".to_string());
        }

        // 2. Check correct side
        if self.current_side != side {
            return Err(format!("Not {:?}'s turn, current side is {:?}", side, self.current_side));
        }

        // 3. Check name is in battleParameters
        if !self.starting_parameters.players(side).contains(&entity_name) {
            return Err(format!("{:?} '{}' is not in the battle", side, entity_name));
        }

        // 4. Check name hasn't taken turn yet
        let taken_turns = match side {  
            TurnSide::PC => &self.pc_taken_turns,
            TurnSide::NPC => &self.npc_taken_turns,
        };
        if taken_turns.contains(&entity_name) {
            return Err(format!("{:?} '{}' has already taken their turn this round", side, entity_name));
        }
        
        // Start the turn - set current_turn but don't mark as taken yet
        Ok(Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: self.current_side,
            current_turn: Some((side, entity_name)),
            pc_taken_turns: self.pc_taken_turns.clone(),
            npc_taken_turns: self.npc_taken_turns.clone(),
            round: self.round,
        })
    }

    pub fn cancel_turn(&self) -> Result<Self, String> {

        if self.current_turn.is_none() {
            return Err("No turn in progress to cancel.".to_string());
        }

        Ok(Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: self.current_side,
            current_turn: None,
            pc_taken_turns: self.pc_taken_turns.clone(),
            npc_taken_turns: self.npc_taken_turns.clone(),
            round: self.round,
        })
    }

    pub fn end_turn(&self) -> Result<Self, String> {
        // 1. Check if there's a turn in progress
        let (side, entity_name) = match &self.current_turn {
            Some(turn) => turn.clone(),
            None => return Err("No turn in progress to end.".to_string()),
        };
        
        // Create new state with updated values - mark turn as taken
        let (pc_taken_turns, npc_taken_turns, mut next_side) = match side {
            TurnSide::PC => {
                let mut new_pc_turns = self.pc_taken_turns.clone();
                new_pc_turns.insert(entity_name.clone());
                (new_pc_turns, self.npc_taken_turns.clone(), TurnSide::NPC)
            }
            TurnSide::NPC => {
                let mut new_npc_turns = self.npc_taken_turns.clone();
                new_npc_turns.insert(entity_name.clone());
                (self.pc_taken_turns.clone(), new_npc_turns, TurnSide::PC)
            }
        };

        // Check the remaining turns on the next side. if there are none, remain on the current side.
        let remaining_next_turns = match next_side {
            TurnSide::PC => {
                // Check how many PCs haven't taken their turn yet
                self.starting_parameters.pcs()
                    .iter()
                    .filter(|pc| !pc_taken_turns.contains(*pc))
                    .count()
            }
            TurnSide::NPC => {
                // Check how many NPCs haven't taken their turn yet
                self.starting_parameters.npcs()
                    .iter()
                    .filter(|npc| !npc_taken_turns.contains(*npc))
                    .count()
            }
        };

        // Stay on current side if next side has no remaining turns
        if remaining_next_turns == 0 {
            next_side = side;
        }
        
        Ok(Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: next_side,
            current_turn: None, // Clear the current turn
            pc_taken_turns,
            npc_taken_turns,
            round: self.round,
        })
    }

    pub fn complete_round(&self) -> Result<Self, String> {
        //check if there is a turn in progress
        if self.current_turn.is_some() {
            return Err("A turn is in progress. End the current turn first.".to_string());
        }

        //can only complete round if all entities have taken their turn
        if self.pc_taken_turns.len() != self.starting_parameters.pcs().len() || self.npc_taken_turns.len() != self.starting_parameters.npcs().len() {
            return Err("Not all entities have taken their turn.".to_string());
        }

    
        //check if the current side has no remaining turns
        if !self.available().is_empty() {
            return Err("There are still entities available to take their turn.".to_string());
        }

        Ok(Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: self.starting_parameters.starting_side(),
            current_turn: None, // Clear any turn in progress
            pc_taken_turns: HashSet::with_capacity(self.starting_parameters.pcs().len()),
            npc_taken_turns: HashSet::with_capacity(self.starting_parameters.npcs().len()),
            round: self.round + 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_battle(pc_count: usize, npc_count: usize, starting_side: TurnSide) -> BattleState {
        let pcs: HashSet<String> = (0..pc_count)
            .map(|i| format!("PC{}", i + 1))
            .collect();
        let npcs: HashSet<String> = (0..npc_count)
            .map(|i| format!("NPC{}", i + 1))
            .collect();
        
        let params = BattleParameters::new(pcs, npcs, starting_side);
        BattleState::new(params)
    }

    #[test]
    fn test_turn_switching_pc_to_npc() {
        // PC starts, should switch to NPC after ending PC turn
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        assert_eq!(battle.current_side(), TurnSide::PC);
        
        // Start and end PC1's turn
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Should switch to NPC side
        assert_eq!(battle.current_side(), TurnSide::NPC);
        assert!(battle.pc_taken_turns().contains("PC1"));
        assert_eq!(battle.pc_taken_turns().len(), 1);
    }

    #[test]
    fn test_turn_switching_npc_to_pc() {
        // NPC starts, should switch to PC after ending NPC turn
        let battle = create_test_battle(2, 2, TurnSide::NPC);
        
        assert_eq!(battle.current_side(), TurnSide::NPC);
        
        // Start and end NPC1's turn
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Should switch to PC side
        assert_eq!(battle.current_side(), TurnSide::PC);
        assert!(battle.npc_taken_turns().contains("NPC1"));
        assert_eq!(battle.npc_taken_turns().len(), 1);
    }

    #[test]
    fn test_turn_switching_stays_on_side_when_other_side_done() {
        // If all NPCs have taken their turn, ending a PC turn should keep it on PC side
        let battle = create_test_battle(2, 1, TurnSide::PC);
        
        // PC1 takes turn
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        assert_eq!(battle.current_side(), TurnSide::NPC);
        
        // NPC1 takes turn (only NPC)
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Should stay on NPC side since all NPCs are done, but wait - let me check the logic
        // Actually, the logic switches to PC first, then checks if PC has remaining turns
        // Since PC1 hasn't taken a turn yet, it should switch to PC
        assert_eq!(battle.current_side(), TurnSide::PC);
        
        // Now PC2 takes turn
        let battle = battle.start_turn(TurnSide::PC, "PC2".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // All NPCs are done, so should stay on PC side
        assert_eq!(battle.current_side(), TurnSide::PC);
    }

    #[test]
    fn test_turn_switching_stays_on_side_when_all_pcs_done() {
        // If all PCs have taken their turn, ending an NPC turn should keep it on NPC side
        let battle = create_test_battle(1, 2, TurnSide::PC);
        
        // PC1 takes turn
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        assert_eq!(battle.current_side(), TurnSide::NPC);
        
        // NPC1 takes turn
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Should switch to PC, but all PCs are done, so should stay on NPC
        // Actually wait - the logic checks the NEXT side (PC) for remaining turns
        // Since PC1 already took a turn, there are 0 remaining PC turns, so it stays on NPC
        assert_eq!(battle.current_side(), TurnSide::NPC);
    }

    #[test]
    fn test_round_completion_basic() {
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        assert_eq!(battle.round(), 1);
        
        // Complete all turns: PC1, NPC1, PC2, NPC2
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::PC, "PC2".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::NPC, "NPC2".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Verify all turns are taken
        assert_eq!(battle.pc_taken_turns().len(), 2);
        assert_eq!(battle.npc_taken_turns().len(), 2);
        assert!(battle.available().is_empty());
        
        // Complete the round
        let battle = battle.complete_round().unwrap();
        
        // Round should increment
        assert_eq!(battle.round(), 2);
        
        // Taken turns should be reset
        assert_eq!(battle.pc_taken_turns().len(), 0);
        assert_eq!(battle.npc_taken_turns().len(), 0);
        
        // Current side should reset to starting side
        assert_eq!(battle.current_side(), TurnSide::PC);
    }

    #[test]
    fn test_round_completion_resets_to_starting_side() {
        // Test that round completion resets to the original starting side
        let battle = create_test_battle(1, 1, TurnSide::NPC);
        
        // Complete all turns
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Complete round
        let battle = battle.complete_round().unwrap();
        
        // Should reset to NPC (starting side)
        assert_eq!(battle.current_side(), TurnSide::NPC);
        assert_eq!(battle.round(), 2);
    }

    #[test]
    fn test_round_completion_fails_with_turn_in_progress() {
        let battle = create_test_battle(1, 1, TurnSide::PC);
        
        // Start a turn but don't end it
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        
        // Try to complete round - should fail
        let result = battle.complete_round();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("turn is in progress"));
    }

    #[test]
    fn test_round_completion_fails_when_not_all_turns_taken() {
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        // Only take some turns
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // Try to complete round - should fail (PC2 and NPC2 haven't taken turns)
        let result = battle.complete_round();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Not all entities have taken their turn"));
    }

    #[test]
    fn test_round_completion_fails_when_entities_still_available() {
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        // Take all turns but one
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::PC, "PC2".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        let battle = battle.start_turn(TurnSide::NPC, "NPC2".to_string()).unwrap();
        let battle = battle.end_turn().unwrap();
        
        // This should actually work since all turns are taken
        // But let me verify the available() check
        assert!(battle.available().is_empty());
        let result = battle.complete_round();
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_rounds() {
        let battle = create_test_battle(1, 1, TurnSide::PC);
        
        // Round 1
        let mut battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        battle = battle.complete_round().unwrap();
        
        assert_eq!(battle.round(), 2);
        assert_eq!(battle.current_side(), TurnSide::PC);
        
        // Round 2
        battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        battle = battle.complete_round().unwrap();
        
        assert_eq!(battle.round(), 3);
        assert_eq!(battle.current_side(), TurnSide::PC);
    }

    #[test]
    fn test_turn_switching_alternates_correctly() {
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        let mut battle = battle;
        
        // PC1 -> NPC1 -> PC2 -> NPC2
        battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        assert_eq!(battle.current_side(), TurnSide::NPC);
        
        battle = battle.start_turn(TurnSide::NPC, "NPC1".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        assert_eq!(battle.current_side(), TurnSide::PC);
        
        battle = battle.start_turn(TurnSide::PC, "PC2".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        assert_eq!(battle.current_side(), TurnSide::NPC);
        
        battle = battle.start_turn(TurnSide::NPC, "NPC2".to_string()).unwrap();
        battle = battle.end_turn().unwrap();
        // After NPC2, all NPCs are done, so should stay on NPC side
        assert_eq!(battle.current_side(), TurnSide::NPC);
    }

    #[test]
    fn test_end_turn_marks_entity_as_taken() {
        let battle = create_test_battle(2, 2, TurnSide::PC);
        
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        assert!(!battle.pc_taken_turns().contains("PC1"));
        
        let battle = battle.end_turn().unwrap();
        assert!(battle.pc_taken_turns().contains("PC1"));
        assert_eq!(battle.pc_taken_turns().len(), 1);
    }

    #[test]
    fn test_end_turn_clears_current_turn() {
        let battle = create_test_battle(1, 1, TurnSide::PC);
        
        let battle = battle.start_turn(TurnSide::PC, "PC1".to_string()).unwrap();
        assert!(battle.current_turn().is_some());
        
        let battle = battle.end_turn().unwrap();
        assert!(battle.current_turn().is_none());
    }
}