use serde::{Deserialize, Serialize};

use crate::game::{Game, Position};

use super::{entity_traits::EntityDamage, Entity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: u64,
    pub type_id: u64,
    pub value: u64,
    pub pos: Position,
    pub hp: u64,
}

impl Entity for Resource {
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

impl EntityDamage for Resource {
    fn damage(&mut self, damage: u64) -> bool {
        if self.hp <= damage {
            self.hp = 0;
            return true;
        }
        self.hp -= damage;
        return false;
    }
}

impl Resource {
    pub fn new(game: &Game, type_id: u64, value: u64, pos: Position, hp: u64) -> Resource {
        Resource {
            id: Game::generate_u64_id(game),
            type_id,
            value,
            pos,
            hp,
        }
    }
}
