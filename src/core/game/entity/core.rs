use serde::{Deserialize, Serialize};

use crate::game::{Game, Position};

use super::{entity_traits::EntityTeam, Entity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Core {
    pub id: u64,
    pub team_id: u64,
    pub pos: Position,
    pub hp: u64,
}

impl Entity for Core {
    fn id(&self) -> u64 {
        self.id
    }
    fn pos(&self) -> &Position {
        &self.pos
    }
    fn hp(&self) -> u64 {
        self.hp
    }
}

impl EntityTeam for Core {
    fn team_id(&self) -> u64 {
        self.team_id
    }
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
