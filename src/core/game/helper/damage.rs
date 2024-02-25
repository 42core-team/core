use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Damage {
    pub attacker_id: u64,
    pub target_id: u64,
    pub damage: u64,
}

impl Damage {
    pub fn new(attacker_id: u64, target_id: u64, damage: u64) -> Self {
        Self {
            attacker_id,
            target_id,
            damage,
        }
    }
}
