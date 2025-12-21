use std::collections::HashSet;

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
    pcs: HashSet<String>,
    npcs: HashSet<String>,
    starting_side: TurnSide,
}

#[allow(dead_code)]
impl BattleParameters {
    pub fn new(pcs: HashSet<String>, npcs: HashSet<String>, starting_side: TurnSide) -> Self {
        Self {
            pcs,
            npcs,
            starting_side,
        }
    }
    
    pub fn players(&self, turn: TurnSide) -> &HashSet<String> {
        match turn {
            TurnSide::NPC => &self.npcs,
            TurnSide::PC => &self.pcs,
        }
    }

    pub fn pcs(&self) -> &HashSet<String> {
        &self.pcs
    }
    
    pub fn npcs(&self) -> &HashSet<String> {
        &self.npcs
    }
    
    pub fn starting_side(&self) -> TurnSide {
        self.starting_side
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

#[allow(dead_code)]
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

    pub fn all_pcs(&self) -> &HashSet<String> {
        self.starting_parameters.pcs()
    }

    pub fn all_npcs(&self) -> &HashSet<String> {
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

    pub fn available(&self) -> HashSet<String> {
        match self.current_side {
            TurnSide::PC => {
                // PCs can act, exclude those who already took their turn or are currently taking it
                let mut available: HashSet<String> = self.starting_parameters.pcs()
                    .difference(&self.pc_taken_turns)
                    .cloned()
                    .collect();
                
                // Remove the entity currently taking their turn
                if let Some((TurnSide::PC, ref name)) = self.current_turn {
                    available.remove(name);
                }
                
                available
            }
            TurnSide::NPC => {
                // NPCs can act, exclude those who already took their turn or are currently taking it
                let mut available: HashSet<String> = self.starting_parameters.npcs()
                    .difference(&self.npc_taken_turns)
                    .cloned()
                    .collect();
                
                // Remove the entity currently taking their turn
                if let Some((TurnSide::NPC, ref name)) = self.current_turn {
                    available.remove(name);
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
                    .difference(&pc_taken_turns)
                    .count()
            }
            TurnSide::NPC => {
                // Check how many NPCs haven't taken their turn yet
                self.starting_parameters.npcs()
                    .difference(&npc_taken_turns)
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

    pub fn complete_round(&self) -> Self {
        Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: self.starting_parameters.starting_side(),
            current_turn: None, // Clear any turn in progress
            pc_taken_turns: HashSet::with_capacity(self.starting_parameters.pcs().len()),
            npc_taken_turns: HashSet::with_capacity(self.starting_parameters.npcs().len()),
            round: self.round + 1,
        }
    }
}