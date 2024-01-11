use serde::Deserialize;

use super::{State, GameConfig};


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Message {
    State(State),
    GameConfig(GameConfig),
}

impl Message {
    pub fn from_state(state: &State) -> Self {
        Message::State(state.clone())
    }
    pub fn from_game_config(game_config: &GameConfig) -> Self {
        Message::GameConfig(game_config.clone())
    }
}