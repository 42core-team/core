use crate::game::{Core, Resource, Unit};

pub enum Target {
    Unit(Unit),
    Resource(Resource),
    Core(Core),
    None,
}
