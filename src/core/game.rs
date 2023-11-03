use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Game {
	pub map: Map
}

#[derive(Debug, Deserialize)]
pub struct Map {
	// width: u128,
	// height: u128,
	// teams: Vec<Team>,
	pub entities: Vec<Entity>
}

#[derive(Debug, Deserialize)]
pub enum Entity {
	Core(Core)
}

#[derive(Debug, Deserialize)]
pub struct Core {
	pub team_id: u8,
	pub x: u128,
	pub y: u128
}