use crate::game::{Unit, Resource, Core};

pub enum Target {
	Unit(Unit),
    Resource(Resource),
    Core(Core),
    None
}

