use serde::{Deserialize, Serialize};

use crate::game::{Game, GameConfig, Position, UnitConfig};

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
    fn deal_dmg(&mut self, dmg: i32, config: &GameConfig) -> bool {
        let max_hp = config.core_hp;

        let mut new_hp = self.hp as i32 - dmg;
        if new_hp < 0 {
            self.hp = 0;
            return true;
        }
        if new_hp > max_hp as i32 {
            new_hp = max_hp as i32;
        }

        self.hp = new_hp as u64;
        false
    }
}

impl EntityConfig for Core {
    fn config_dmg(&self, config: UnitConfig) -> i32 {
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
