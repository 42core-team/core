use serde::{Deserialize, Serialize};

use super::{Game, GameConfig};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub hp: u64,
    pub x: u64,
    pub y: u64,
    pub team_id: u64,
}

impl Unit {
	pub fn new(team_id: u64, type_id: u64, x: u64, y: u64) -> Self {
		Unit {
			id: Game::generate_u64_id(),
			type_id,
			hp: GameConfig::get_unit_config_by_type_id(type_id).unwrap().hp,
			x,
			y,
			team_id,
		}
	}
}
