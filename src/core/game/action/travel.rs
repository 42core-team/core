use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Travel {
    pub id: u64,
    pub x: u64,
    pub y: u64,
}
