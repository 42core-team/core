use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    pub id: u64,
    pub value: u64,
    pub x: u64,
    pub y: u64,
    pub hp: u64,
}
