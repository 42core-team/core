use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Login {
    pub id: u64
}
