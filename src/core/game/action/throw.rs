use crate::game::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Throw {
    pub unit_id: u64,
    pub target_pos: Position,
}
