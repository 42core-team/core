use crate::game::{Position, UnitConfig};

pub trait Entity {
    fn id(&self) -> u64;
    fn team_id(&self) -> u64;
    fn pos(&self) -> &Position;
    fn hp(&self) -> u64;
    fn deal_damage(&mut self, damage: u64) -> bool;
}

pub trait EntityConfig {
    fn damage(&self, config: UnitConfig) -> u64;
}
