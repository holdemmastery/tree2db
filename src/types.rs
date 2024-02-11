use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Position {
    SB,
    BB,
    UTG,
    HJ,
    CO,
    BTN,
}

#[derive(Serialize, Deserialize)]
pub enum Player {
    Hero,
    Villain,
}

#[derive(Serialize, Deserialize)]
pub enum MoveOption {
    Fold,
    Check,
    Call,
    Bet33,
    Bet50,
    Bet75,
    Bet100,
    Bet175,
    Raise75,
}

#[derive(Serialize, Deserialize)]
pub enum Street {
    PreFlop,
    Flop,
    Turn,
    River,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub street: Street,
    pub actor: Player,
    pub option: MoveOption,
}

#[derive(Serialize, Deserialize)]
pub struct OptionEVPair {
    pub option: MoveOption,     
    pub ev: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Problem {
    pub time_created: u64,
    pub hero_position: Position,
    pub problem_elo: u16,
    pub villain_position: Position,
    pub hole_cards: String,
    pub board: String,
    pub action_history: Vec<Action>,
    pub river_option_evs: Vec<OptionEVPair>,
}
