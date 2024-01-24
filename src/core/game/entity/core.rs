use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Core {
    pub id: u64,
    pub team_id: u64,
    pub x: u64,
    pub y: u64,
    pub hp: u64,
}

impl Core {
    pub fn new(game: &Game, team_id: u64, x: u64, y: u64, hp: u64) -> Self {
        Core {
            id: Game::generate_u64_id(game),
            team_id,
            x,
            y,
            hp,
        }
    }
}
