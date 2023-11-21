use std::time::Duration;

use uuid::Uuid;

use crate::game::action::Action;

use super::{utils::get_ms, Entity, GameConfig, State, Team, Unit, entity::Core};

#[derive(Debug)]
pub struct Game {
	pub teams: Vec<Team>,
	pub config: GameConfig,
	pub entities: Vec<Entity>,
	pub units: Vec<Unit>,
	pub last_tick_time: u128,
	pub time_since_last_tick: u128,
}

impl Game {
	pub fn new(teams: Vec<Team>) -> Self {
		Game {
			teams,
			config: GameConfig::patch_0_1_0(),
			entities: vec![],
			units: vec![],
			last_tick_time: get_ms(),
			time_since_last_tick: 0,
		}
	}

	pub async fn start(&mut self) {
		loop {
			self.wait_till_next_tick().await;
			println!("TICK");

			let mut team_actions: Vec<(u64, Action)> = vec![];
			
			for team_index in 0..self.teams.len() {
				let team = &mut self.teams[team_index];
				while let Ok(actions) = team.receiver.try_recv() {
					println!("TEAM send action: {:?}", actions);
					for action in actions {
						team_actions.push((team.id, action));
					}
				}
			}
			self.update(team_actions);
			self.send_state().await;
		}
	}

	async fn send_state(&mut self) {
		let state = State::from_game(self);
		for team in self.teams.iter_mut() {
			let state = state.clone();
			match team.sender.send(state).await {
				Ok(_) => {}
				Err(_) => {
					println!("Error sending state to team");
				}
			}
		}
	}

	async fn wait_till_next_tick(&mut self) {
		let min_ms_per_tick: u128 = 3000;

		loop {
			// This is so that it always takes 1ms steps minimum
			if get_ms() <= self.last_tick_time {
				tokio::time::sleep(Duration::from_millis(1)).await;
				continue;
			}

			self.time_since_last_tick = get_ms() - self.last_tick_time;

			if self.time_since_last_tick > min_ms_per_tick {
				self.last_tick_time = self.last_tick_time + self.time_since_last_tick;
				break;
			}

			tokio::time::sleep(Duration::from_millis(((min_ms_per_tick / 2) + 1) as u64)).await;
		}
	}

	pub fn generate_u64_id() -> u64 {
		let uuid = Uuid::default();
	
		let u64_id = u64::from_le_bytes(uuid.as_bytes()[..8].try_into().unwrap());
	
		u64_id
	}
	

	pub fn get_team_by_id(&self, id: u64) -> Option<&Team> {
		for team in self.teams.iter() {
			if team.id == id {
				return Some(team);
			}
		}

		None
	}

	pub fn get_core_by_team_id(&self, team_id: u64) -> Option<&Core> {
		for entity in self.entities.iter() {
			println!("Entity: {:?}", entity);
			match entity {
				Entity::Core(core) => {
					if core.team_id == team_id {
						return Some(core);
					}
				}
				_ => {}
			}
		}
		None
	}

	pub fn create_unit(&mut self, team_id: u64, type_id: u64) {
		println!("Create unit of type {:?} for team with id {:?}", type_id, team_id);
		let team_core = self.get_core_by_team_id(team_id);
		if team_core.is_none() {
			println!("Core of team with id {:?} not found", team_id);
			return;
		}
		let team_core = team_core.unwrap();
		let unit = Unit::new(team_id, type_id, team_core.x, team_core.y);
		self.units.push(unit);
	}

	///
	/// Handel the update of the game
	/// 
	/// a valid json to send with netcat is:
	/// [{"Create":{"type_id":3}},{"Travel":{"id":1,"x":2,"y":3}},{"Attack":{"attacker_id":1,"target_id":2}}]
	/// 
	/// To uns netcat:
	/// ```sh
	/// nc localhost 4242
	/// ```
	/// then paste the json and press enter
	/// 
	/// You need at least two netcat instances to start a game
	/// 
	pub fn update(&mut self, team_actions: Vec<(u64, Action)>) {
		for (team_id, action) in team_actions {
			match action {
				Action::Create(create) => {
					self.create_unit(team_id, create.type_id);
				}
				Action::Attack(attack) => {
					println!("Attack: {:?}", attack);
				}
				Action::Travel(travel) => {
					println!("Travel: {:?}", travel);
				}
			}
		}
	}
}
