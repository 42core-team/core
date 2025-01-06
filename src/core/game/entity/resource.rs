use serde::{Deserialize, Serialize};

use crate::game::{Game, GameConfig, Position, UnitConfig};

use super::{entity_traits::EntityConfig, Entity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: u64,
    pub type_id: u64,
    pub pos: Position,
    pub hp: u64,
}

impl Entity for Resource {
    fn id(&self) -> u64 {
        self.id
    }
    fn team_id(&self) -> u64 {
        0
    }
    fn pos(&self) -> &Position {
        &self.pos
    }
    fn hp(&self) -> u64 {
        self.hp
    }
    fn deal_dmg(&mut self, dmg: i32, config: &GameConfig) -> bool {
        let max_hp = config
            .resources
            .iter()
            .find(|r| r.type_id == self.type_id)
            .unwrap()
            .hp;

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

impl EntityConfig for Resource {
    fn config_dmg(&self, config: UnitConfig) -> i32 {
        return config.dmg_resource;
    }
}

impl Resource {
    pub fn new(game: &Game, type_id: u64, pos: Position, hp: u64) -> Resource {
        Resource {
            id: Game::generate_u64_id(game),
            type_id,
            pos,
            hp,
        }
    }

    pub fn balance_from_dmg(&self, game_config: &GameConfig, mut dmg: i32) -> i32 {
        let resource_config = game_config
            .resources
            .iter()
            .find(|r| r.type_id == self.type_id)
            .expect("Resource config not found");

        if self.hp as i32 - dmg < 0 {
            dmg = self.hp as i32;
        }

        let balance_factor = resource_config.balance_value as f64 / resource_config.hp as f64;
        (dmg as f64 * balance_factor) as i32
    }
}
