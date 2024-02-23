use crate::game::{
    entity::{entity_traits::EntityDamage, Unit},
    Core, Entity, EntityTeam, Resource,
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
}

impl EntityTeam for Target {
    fn team_id(&self) -> u64 {
        match self {
            Target::Unit(u) => u.team_id(),
            Target::Resource(_) => 0,
            Target::Core(c) => c.team_id(),
        }
    }
}

impl Target {
    fn damage(&mut self, dmg_unit: u64, dmg_resource: u64, dmg_core: u64) -> bool {
        match self {
            Target::Unit(u) => u.damage(dmg_unit),
            Target::Resource(r) => r.damage(dmg_resource),
            Target::Core(c) => c.damage(dmg_core),
        }
    }
}
