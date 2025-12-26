use std::fs;
use std::path::Path;
use indexmap::{IndexMap};
use odsr_engine::{CombatParameters, CombatState, TurnSide, entity::{Entity, EntityDefinition}};

pub enum CombatMode {
    Setup(CombatParameters),
    Active(CombatState),
}

pub struct App {
    pub monster_definitions: IndexMap<String, EntityDefinition>,
    pub hero_definitions: IndexMap<String, EntityDefinition>,
    pub entities: IndexMap<String, Entity>,
    pub state: Option<CombatMode>,
    pub log: Vec<String>,
    pub log_view_expanded: bool,
    pub input_mode: InputMode,
    pub text_input: Option<TextInput>,
}

pub enum InputMode {
    CreatingCombat,
    TakingTurn,
    TextInput,
    RemovingEntity,
    SelectingHeroDefinition,
    SelectingMonsterDefinition,
}

#[derive(Copy, Clone)]
pub enum TextInputType {
    NPCName,
    PCName,
}

pub struct TextInput {
    pub buffer: String,
    pub input_type: TextInputType,
    pub selected_definition: Option<String>, // For NPCs: the monster definition name
}

/// Load entity definitions from the content/monsters/ directory
fn load_entity_definitions(dir: &Path) -> Result<IndexMap<String, EntityDefinition>, String> {
    if !dir.exists() {
        return Err(format!("Directory {} does not exist", dir.display()));
    }
    
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Err(format!("Failed to read directory {}", dir.display())),
    };

    let mut definitions = IndexMap::new();
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            return Err(format!("File {} is not a JSON file", path.display()));
        }
        
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Err(format!("Failed to read file {}", path.display())),
        };
        
        let definition: EntityDefinition = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(_) => return Err(format!("Failed to parse file {}", path.display())),
        };
        
        definitions.insert(definition.name.clone(), definition);
    }
    
    Ok(definitions)
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
        
        let monster_definitions = match load_entity_definitions(Path::new("content/monsters")) {
            Ok(definitions) => definitions,
            Err(e) => return Err(e),
        };
        let hero_definitions = match load_entity_definitions(Path::new("content/heroes")) {
            Ok(definitions) => definitions,
            Err(e) => return Err(e),
        };
        
        let app = App {
            monster_definitions,
            hero_definitions,
            entities: IndexMap::new(),
            state: Some(CombatMode::Setup(combat_params)),
            log: vec!["Welcome! Press 'n' to start combat, or 'q' to quit.".to_string()],
            log_view_expanded: false,
            input_mode: InputMode::CreatingCombat,
            text_input: None,
        };
        
        Ok(app)
    }
}

