use crate::game::{
    entity::{EntityConfig, Unit},
    Core, Entity, GameConfig, Resource, UnitConfig,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Unit(Unit),
    Resource(Resource),
    Core(Core),
}

impl Entity for Target {
    fn id(&self) -> u64 {
        match self {
            Target::Unit(u) => u.id(),
            Target::Resource(r) => r.id(),
            Target::Core(c) => c.id(),
        }
    }
    fn team_id(&self) -> u64 {
        match self {
            Target::Unit(u) => u.team_id(),
            Target::Resource(_) => 0,
            Target::Core(c) => c.team_id(),
        }
    }
    fn pos(&self) -> &crate::game::Position {
        match self {
            Target::Unit(u) => u.pos(),
            Target::Resource(r) => r.pos(),
            Target::Core(c) => c.pos(),
        }
    }
    fn hp(&self) -> u64 {
        match self {
            Target::Unit(u) => u.hp(),
            Target::Resource(r) => r.hp(),
            Target::Core(c) => c.hp(),
        }
    }
    fn deal_dmg(&mut self, dmg: i32, config: &GameConfig) -> bool {
        match self {
            Target::Unit(u) => u.deal_dmg(dmg, config),
            Target::Resource(r) => r.deal_dmg(dmg, config),
            Target::Core(c) => c.deal_dmg(dmg, config),
        }
    }
}

impl EntityConfig for Target {
    fn config_dmg(&self, config: UnitConfig) -> i32 {
        match self {
            Target::Unit(u) => u.config_dmg(config),
            Target::Resource(r) => r.config_dmg(config),
            Target::Core(c) => c.config_dmg(config),
        }
    }
}
