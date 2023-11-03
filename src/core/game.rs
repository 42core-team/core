use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Game {
	pub map: Map
}

#[derive(Debug, Deserialize)]
pub struct Map {
	pub width: u128,
	pub height: u128,
	pub teams: Vec<Team>,
	pub entities: Vec<Entity>
}

#[derive(Debug, Deserialize)]
pub struct Team {
	pub id: u8,
	pub name: String
}

#[derive(Debug, Deserialize)]
pub enum Entity {
	Core(Core),
	Unit(Unit),
	Resource(Resource)
}

#[derive(Debug, Deserialize)]
pub struct Core {
	pub team_id: u8,
	pub x: u128,
	pub y: u128
}

#[derive(Debug, Deserialize)]
pub struct Resource {
	pub x: u128,
	pub y: u128,
	pub value: u8
}

#[derive(Debug, Deserialize)]
pub enum Unit {
	Warrior(Warrior),
	Worker(Worker)
}

#[derive(Debug, Deserialize)]
pub struct Warrior {
	pub team_id: u8,
	pub x: u128,
	pub y: u128
}

#[derive(Debug, Deserialize)]
pub struct Worker {
	pub team_id: u8,
	pub x: u128,
	pub y: u128
}