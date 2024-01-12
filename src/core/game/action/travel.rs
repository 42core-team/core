use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Travel {
    id: u64,
    x: u64,
    y: u64,
}
