use serde::{Deserialize, Serialize};

use crate::game::{Game, Position, UnitConfig};

use super::{entity_traits::EntityConfig, Entity};

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
    fn team_id(&self) -> u64 {
        self.team_id
    }
    fn pos(&self) -> &Position {
        &self.pos
    }
    fn hp(&self) -> u64 {
        self.hp
    }
}

impl EntityConfig for Core {
    fn damage(&self, config: UnitConfig) -> u64 {
        return config.dmg_core;
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
