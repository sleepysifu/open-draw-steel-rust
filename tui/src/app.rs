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
    pub message: String,
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
fn load_entity_definitions(dir: &Path) -> IndexMap<String, EntityDefinition> {
    let mut definitions = IndexMap::new();
    
    
    if !dir.exists() {
        return definitions;
    }
    
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return definitions,
    };
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        
        let definition: EntityDefinition = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(_) => continue,
        };
        
        definitions.insert(definition.name.clone(), definition);
    }
    
    definitions
}

impl Default for App {
    fn default() -> App {
        let combat_params = CombatParameters::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            TurnSide::PC,
        );
        
        let monster_definitions = load_entity_definitions(Path::new("content/monsters"));
        let hero_definitions = load_entity_definitions(Path::new("content/heroes"));
        
        App {
            monster_definitions,
            hero_definitions,
            entities: IndexMap::new(),
            state: Some(CombatMode::Setup(combat_params)),
            message: "Welcome! Press 'n' to start combat, or 'q' to quit.".to_string(),
            input_mode: InputMode::CreatingCombat,
            text_input: None,
        }
    }
}

