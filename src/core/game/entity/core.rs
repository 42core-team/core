use serde::{Deserialize, Serialize};

use crate::game::{Game, Position};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Core {
    pub id: u64,
    pub team_id: u64,
    pub pos: Position,
    pub hp: u64,
}

impl Core {
    pub fn new(game: &Game, team_id: u64, pos: Position, hp: u64) -> Self {
        Core {
            id: Game::generate_u64_id(game),
            team_id,
            pos,
            hp,
        }
    }
}
