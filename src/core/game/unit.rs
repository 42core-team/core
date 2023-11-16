use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub hp: u64,
    pub x: u64,
    pub y: u64,
    pub team_id: u64,
}
