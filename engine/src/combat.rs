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

    pub fn available(&self) -> HashSet<String> {
        match self.current_side {
            TurnSide::PC => {
                // PCs can act, exclude those who already took their turn
                self.starting_parameters.pcs()
                    .difference(&self.pc_taken_turns)
                    .cloned()
                    .collect()
            }
            TurnSide::NPC => {
                // NPCs can act, exclude those who already took their turn
                self.starting_parameters.npcs()
                    .difference(&self.npc_taken_turns)
                    .cloned()
                    .collect()
            }
        }
    }

    pub fn take_turn(&self, side: TurnSide, entity_name: String) -> Result<Self, String> {
        // 1. Check correct side
        if self.current_side != side {
            return Err(format!("Not {:?}'s turn, current side is {:?}", side, self.current_side));
        }

        // 2. Check name is in battleParameters
        if !self.starting_parameters.players(side).contains(&entity_name) {
            return Err(format!("{:?} '{}' is not in the battle", side, entity_name));
        }

        // 3. Check name hasn't taken turn yet
        let taken_turns = match side {  
            TurnSide::PC => &self.pc_taken_turns,
            TurnSide::NPC => &self.npc_taken_turns,
        };
        if taken_turns.contains(&entity_name) {
            return Err(format!("{:?} '{}' has already taken their turn this round", side, entity_name));
        }
        
        // Create new state with updated values
        let (pc_taken_turns, npc_taken_turns, mut next_side) = match side {
            TurnSide::PC => {
                let mut new_pc_turns = self.pc_taken_turns.clone();
                new_pc_turns.insert(entity_name);
                (new_pc_turns, self.npc_taken_turns.clone(), TurnSide::NPC)
            }
            TurnSide::NPC => {
                let mut new_npc_turns = self.npc_taken_turns.clone();
                new_npc_turns.insert(entity_name);
                (self.pc_taken_turns.clone(), new_npc_turns, TurnSide::PC)
            }
        };

        // check the remaining turns on the next side. if there are none, remain on the current side.
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

        //stay on current side
        if remaining_next_turns == 0 {
            next_side = side;
        }
        
        Ok(Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: next_side,
            pc_taken_turns,
            npc_taken_turns,
            round: self.round,
        })
    }

    pub fn complete_round(&self) -> Self {
        Self {
            starting_parameters: self.starting_parameters.clone(),
            current_side: self.starting_parameters.starting_side(),
            pc_taken_turns: HashSet::with_capacity(self.starting_parameters.pcs().len()),
            npc_taken_turns: HashSet::with_capacity(self.starting_parameters.npcs().len()),
            round: self.round + 1,
        }
    }
}