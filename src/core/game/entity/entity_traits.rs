use crate::game::{GameConfig, Position, UnitConfig};

pub trait Entity {
    fn id(&self) -> u64;
    fn team_id(&self) -> u64;
    fn pos(&self) -> &Position;
    fn hp(&self) -> u64;
    fn deal_dmg(&mut self, dmg: i32, config: &GameConfig) -> bool;
}

pub trait EntityConfig {
    fn config_dmg(&self, config: UnitConfig) -> i32;
}
