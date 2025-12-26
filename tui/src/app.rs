use std::path::Path;
use indexmap::{IndexMap};
use odsr_engine::{Ability, CombatParameters, CombatState, TurnSide, entity::{Entity, EntityDefinition}, fs::load_set};

pub enum CombatMode {
    Setup(CombatParameters),
    Active(CombatState),
}

pub struct Definitions {
    pub monsters: IndexMap<String, EntityDefinition>,
    pub heroes: IndexMap<String, EntityDefinition>,
    pub abilities: IndexMap<String, Ability>,
}

impl Definitions {
    pub fn new() -> Result<Self, String> {
        
        let monster_definitions = match load_set::<EntityDefinition>(Path::new("content/monsters")) {
            Ok(definitions) => definitions,
            Err(e) => return Err(e),
        };
        let hero_definitions = match load_set::<EntityDefinition>(Path::new("content/heroes")) {
            Ok(definitions) => definitions,
            Err(e) => return Err(e),
        };
        let abilities = match load_set::<Ability>(Path::new("content/abilities")) {
            Ok(abilities) => abilities,
            Err(e) => return Err(e),
        };
        Ok(Self { monsters: monster_definitions, heroes: hero_definitions, abilities: abilities })
    
    }
}

pub struct App {
    pub definitions: Definitions,
    pub entities: IndexMap<String, Entity>,
    pub state: Option<CombatMode>,
    pub log: Vec<String>,
    pub log_view_expanded: bool,
    pub input_mode: InputMode,
    pub text_input: Option<TextInput>,
    pub selected_ability: Option<String>, // Ability name selected for use
}

pub enum InputMode {
    CreatingCombat,
    TakingTurn,
    TextInput,
    RemovingEntity,
    SelectingHeroDefinition,
    SelectingMonsterDefinition,
    SelectingAbility,
    SelectingTarget,
}

#[derive(Copy, Clone)]
pub enum TextInputType {
    NPCName,
    PCName,
}

pub struct TextInput {
    pub buffer: String,
    pub input_type: TextInputType,
    pub selected_definition: Option<String>,
}


impl App {
    /// Append a message to the log buffer
    pub fn log(&mut self, message: String) {
        self.log.push(message);
        // Keep log size reasonable (last 1000 messages)
        if self.log.len() > 1000 {
            self.log.remove(0);
        }
    }
    
    /// Get the last N log messages
    pub fn last_log_messages(&self, n: usize) -> Vec<String> {
        let start = self.log.len().saturating_sub(n);
        self.log[start..].to_vec()
    }
}

impl App {
    pub fn new() -> Result<App, String> {
        let combat_params = CombatParameters::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            TurnSide::PC,
        );
        
        let definitions = match Definitions::new() {
            Ok(definitions) => definitions,
            Err(e) => return Err(e),
        };
        
        let app = App {
            definitions: definitions,
            entities: IndexMap::new(),
            state: Some(CombatMode::Setup(combat_params)),
            log: vec!["Welcome! Press 'n' to start combat, or 'q' to quit.".to_string()],
            log_view_expanded: false,
            input_mode: InputMode::CreatingCombat,
            text_input: None,
            selected_ability: None,
        };
        
        Ok(app)
    }
}

