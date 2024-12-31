use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Dmg {
    pub amount: i32,
    pub attacker_id: u64,
    pub target_id: u64,
}

impl Dmg {
    pub fn new(attacker_id: u64, target_id: u64, dmg: i32) -> Self {
        Self {
            attacker_id,
            target_id,
            amount: dmg,
        }
    }
}
