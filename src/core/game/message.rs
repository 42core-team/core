use serde::Deserialize;

use super::{action::Action, config::GameConfigWithId, Login, State};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum Message {
    VecAction(Vec<Action>),
    State(State),
    GameConfigWithId(GameConfigWithId),
    Login(Login),
}

impl Message {
    pub fn from_state(state: &State) -> Self {
        Message::State(state.clone())
    }

    pub fn from_game_config(game_config: &GameConfigWithId) -> Self {
        Message::GameConfigWithId(game_config.clone())
    }

    pub fn from_vec_action(vec_action: Vec<Action>) -> Self {
        Message::VecAction(vec_action)
    }

    pub fn from_login(login: &Login) -> Self {
        Message::Login(login.clone())
    }
}
