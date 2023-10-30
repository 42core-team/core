use std;

pub enum Troop_Type {
	WORKER,
	WARRIOR
}

trait action_function {

	fn perform_action(&mut self);

}

pub struct Troop <T: action_function> {
	name: String,
	troop_type: Troop_Type,
	health: u64,
	dps: u64,
	position: Vector2,
	team: u32,
	action: T
}

pub struct Core {
	team_id: u32,
	position: Vector2,
	health: u64
}

pub struct Building {
	team_id: u32,
	position: Vector2,
	health: u64
}

pub struct Team {
	team_name: String,
	team_id: u32
}