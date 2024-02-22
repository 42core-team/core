use crate::game::{entity::Unit, Core, Resource};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Unit(Unit),
    Resource(Resource),
    Core(Core),
    None,
}
