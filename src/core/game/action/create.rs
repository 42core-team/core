use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Create {
    pub type_id: u64,
}

impl Create {
    pub fn new(type_id: u64) -> Self {
        Self { type_id }
    }
}
