use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Update {
	pub status: u64,
	pub entities: Vec<Entity>,
	pub units: Vec<Unit>
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum Entity {
	Core(Core),
	Resource(Resource)
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Core {
	pub id: u64,
	pub team_id: u64,
	pub x: u64,
	pub y: u64,
	pub hp: u64
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Resource {
	pub id: u64,
	pub value: u64,
	pub x: u64,
	pub y: u64,
	pub hp: u64
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Unit {
	pub id: u64,
	pub type_id: u64,
	pub hp: u64,
	pub x: u64,
	pub y: u64,
	pub team_id: u64
}