use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Attack {
    attacker_id: u64,
    target_id: u64,
}
