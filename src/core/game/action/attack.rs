use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Attack {
	pub attacker_id: u64,
	pub target_id: u64,
}
