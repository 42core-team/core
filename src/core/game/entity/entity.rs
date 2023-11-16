use serde::{Deserialize, Serialize};

use super::{Core, Resource};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Entity {
    Core(Core),
    Resource(Resource),
}
