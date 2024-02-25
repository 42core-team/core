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
}

impl EntityConfig for Resource {
    fn damage(&self, config: UnitConfig) -> u64 {
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

    pub fn balance_from_damage(&self, game_config: &GameConfig, damage: u64) -> u64 {
        let resource_config = game_config
            .resources
            .iter()
            .find(|r| r.type_id == self.type_id)
            .expect("Resource config not found");
        damage * resource_config.balance_value / resource_config.hp
    }
}
