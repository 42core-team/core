use crate::game::{
    entity::{EntityConfig, Unit},
    Core, Entity, Resource, UnitConfig,
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
    fn deal_damage(&mut self, damage: u64) -> bool {
        match self {
            Target::Unit(u) => u.deal_damage(damage),
            Target::Resource(r) => r.deal_damage(damage),
            Target::Core(c) => c.deal_damage(damage),
        }
    }
}

impl EntityConfig for Target {
    fn damage(&self, config: UnitConfig) -> u64 {
        match self {
            Target::Unit(u) => u.damage(config),
            Target::Resource(r) => r.damage(config),
            Target::Core(c) => c.damage(config),
        }
    }
}
