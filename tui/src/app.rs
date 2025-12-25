use odsr_engine::{BattleParameters, BattleState, TurnSide};

pub enum BattleMode {
    Setup(BattleParameters),
    Active(BattleState),
}

pub struct App {
    pub state: Option<BattleMode>,
    pub message: String,
    pub input_mode: InputMode,
    pub text_input: Option<TextInput>,
}

pub enum InputMode {
    CreatingBattle,
    TakingTurn,
    TextInput,
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
        let battle_params = BattleParameters::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            TurnSide::PC,
        );
        
        App {
            state: Some(BattleMode::Setup(battle_params)),
            message: "Welcome! Press 'n' to start the battle, or 'q' to quit.".to_string(),
            input_mode: InputMode::CreatingBattle,
            text_input: None,
        }
    }
}

