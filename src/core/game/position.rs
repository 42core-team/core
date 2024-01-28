use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}
