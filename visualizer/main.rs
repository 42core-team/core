use std::{string, vec};

/**
 * Debug visualizer
 * 
 * legend:
 * 		. -> dirt
 * 		x -> ressource
 * 		w -> worker
 * 		a -> warrior
 * 		C -> Core
 */



struct Ressource {
	x: u128,
	y: u128
}

impl Ressource {
	pub fn new(x: u128, y: u128) -> Self {
		Ressource { x, y }
	}
}

struct Core {
	x: u128,
	y: u128
}

impl Core {
	pub fn new(x: u128, y: u128) -> Self {
		Core { x, y }
	}
}

enum UType {
	Worker,
	Warrior
}

struct Unit {
	x: u128,
	y: u128,
	id: String,
	utype: UType
}

impl Unit {
	pub fn new(x: u128, y: u128, id: String, utype: UType) -> Self {
		Unit { x, y, id, utype }
	}
}

 struct  Entities {
    ressources: Vec<Ressource>,
	cores: Vec<Core>,
	units: Vec<Unit>
}

impl Entities {
	pub fn new(ressources: Vec<Ressource>, cores:Vec<Core>, units:Vec<Unit>) -> Self {
		Entities { ressources, cores, units }
	}
}

struct Team{
	id: String,
	name: String,
	ressources: u16
}

impl Team {
	pub fn new(id: String, name: String, ressources: u16) -> Self {
		Team { id, name, ressources }
	}
}

 struct Map {
    width: u128,
	height: u128,
	teams: Vec<Team>,
	entities: Entities
}

impl Map {
	pub fn new(width: u128, height: u128, teams: Vec<Team>, entities: Entities ) -> Self {
		Map { width, height, teams, entities }
	}
}

fn main() {
	let _resolution: u32 = 1000;
	let ressources: Vec<Ressource> = vec![Ressource::new(40000, 20000), Ressource::new(20000, 40000), Ressource::new(10000, 30000)];
	let cores: Vec<Core> = vec![Core::new(10000, 10000), Core::new(90000, 90000)];
	let units: Vec<Unit> = vec![
		Unit::new(20000, 20000, String::from("1"), UType::Warrior),
		Unit::new(21000, 21000, String::from("2"), UType::Worker),
		Unit::new(80000, 80000, String::from("3"), UType::Warrior),
		Unit::new(81000, 81000, String::from("4"), UType::Worker)
	];
	let team1: Team = Team::new(String::from("1"), String::from("Team1"), 0);
	let team2: Team = Team::new(String::from("2"), String::from("Team2"), 0);
	let teams: Vec<Team> = vec![team1, team2];
	let entities: Entities = Entities::new(ressources, cores, units);
	let map = Map::new(100000, 100000, teams, entities);
	loop {
		for row in 0..map.height {
			for col in 0..map.width {

			}
		}
		print!("{}[2J", 27 as char);
	}
}