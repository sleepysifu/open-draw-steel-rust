use odsr_engine::{CombatParameters, CombatState, TurnSide};

pub enum CombatMode {
    Setup(CombatParameters),
    Active(CombatState),
}

pub struct App {
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
}

pub enum TextInputType {
    NPCName,
    PCName,
}

pub struct TextInput {
    pub buffer: String,
    pub input_type: TextInputType,
}

impl Default for App {
    fn default() -> App {
        let combat_params = CombatParameters::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            TurnSide::PC,
        );
        
        App {
            state: Some(CombatMode::Setup(combat_params)),
            message: "Welcome! Press 'n' to start combat, or 'q' to quit.".to_string(),
            input_mode: InputMode::CreatingCombat,
            text_input: None,
        }
    }
}

