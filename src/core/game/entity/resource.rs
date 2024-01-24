use serde::{Deserialize, Serialize};

use crate::game::Game;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: u64,
    pub type_id: u64,
    pub value: u64,
    pub x: u64,
    pub y: u64,
    pub hp: u64,
}

impl Resource {
    pub fn new(game: &Game, type_id: u64, value: u64, x: u64, y: u64, hp: u64) -> Resource {
        Resource {
            id: Game::generate_u64_id(game),
            type_id,
            value,
            x,
            y,
            hp,
        }
    }
}
