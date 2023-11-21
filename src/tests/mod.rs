#[cfg(test)]
mod tests {
	use lib::game::{Team, Game, GameConfig};


	fn get_fake_game() -> Game{
		let t1 = Team::get_fake_team(0, "Team 1".to_string());
		let t2 = Team::get_fake_team(1, "Team 2".to_string()) ;
		let game = Game::new(vec![t1, t2]);
		game
	}

	#[test]
	///
	/// Test if the fake team creation works
	/// 
	/// The fake team is used to test the game logic
	/// 
	fn test_create_fake_team() {
		let team = Team::get_fake_team(0, "asdf".to_string());
		assert_eq!(team.id, 0);
		assert_eq!(team.name, "asdf");
		assert_eq!(team.balance, 100);
	}

	#[test]
	///
	/// Test if the fake game creation works
	/// 
	/// The fake Game is used to test the game logic
	/// 
	fn test_create_fake_game() {
		let game = get_fake_game();
		assert_eq!(game.teams.len(), 2);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);
	}


	#[test]
	///
	/// Test if a team can create a unit
	/// 
	fn test_create_unit() {
		let mut game = get_fake_game();

		assert_eq!(game.teams.len(), 2);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);
		game.create_unit(0, 1);
		assert_eq!(game.units.len(), 1);
		assert_eq!(game.teams[0].balance, 100 - GameConfig::get_unit_config_by_type_id(1).unwrap().cost);
		// Create another unit for team 0
		game.create_unit(0, 1);
		// second create_unit call fails -> balance to low
		assert_eq!(game.units.len(), 1);
		// balance should not change
		assert_eq!(game.teams[0].balance, 100 - GameConfig::get_unit_config_by_type_id(1).unwrap().cost);
		//same for second team
		game.create_unit(1, 2);
		assert_eq!(game.units.len(), 2);
		assert_eq!(game.teams[1].balance, 50);
		game.create_unit(1, 2);
		assert_eq!(game.units.len(), 3);
		assert_eq!(game.teams[1].balance, 0);
		game.create_unit(1, 2);
		assert_eq!(game.units.len(), 3);
		assert_eq!(game.teams[1].balance, 0);
	}

	#[test]
	///
	/// Test for invalid input in the create_unit function
	/// 
	fn test_invalid_input_create_unit(){
		let mut game = get_fake_game();

		assert_eq!(game.teams.len(), 2);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);

		// invalid team id
		game.create_unit(2, 1);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);

		// invalid unit type id
		game.create_unit(0, 3);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);

		// invalid team id and unit type id
		game.create_unit(2, 3);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);

		// invalid team id and valid unit type id
		game.create_unit(2, 1);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);

		// valid team id and invalid unit type id
		game.create_unit(0, 3);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);
	}

	#[test]
	///
	/// Test the get_team_by_id function
	/// 
	/// the cores are
	/// 0: (2, 2)
	/// 1: (4, 4)
	/// 
	fn test_get_core_by_team_id() {
		let game = get_fake_game();
		
		let core1 = game.get_core_by_team_id(0);
		assert_eq!(core1.unwrap().x, 2);
		assert_eq!(core1.unwrap().y, 2);

		let core2 = game.get_core_by_team_id(1);
		assert_eq!(core2.unwrap().x, 4);
		assert_eq!(core2.unwrap().y, 4);

		let core3 = game.get_core_by_team_id(2);
		assert_eq!(core3, None);
	}

	#[test]
	fn test_get_team_by_id() {
		let game = get_fake_game();
		
		let team1 = game.get_team_by_id(0);
		match team1 {
			Some(team) => {
				assert_eq!(team.name, "Team 1");
			}
			None => {
				assert!(false);
			}
		}

		let team2 = game.get_team_by_id(1);
		match team2 {
			Some(team) => {
				assert_eq!(team.name, "Team 2");
			}
			None => {
				assert!(false);
			}
		}

		let team3 = game.get_team_by_id(2);
		match team3 {
			Some(_) => {
				assert!(false);
			}
			None => {
				assert!(true);
			}
		}
	}

	#[test]
	fn test_get_team_by_id_mut() {
		let mut game = get_fake_game();
		
		let team1 = game.get_team_by_id_mut(0);
		assert_eq!(team1.unwrap().name, "Team 1");

		let team2 = game.get_team_by_id_mut(1);
		assert_eq!(team2.unwrap().name, "Team 2");

		let team3 = game.get_team_by_id_mut(2);
		match team3 {
			Some(_) => {
				assert!(false);
			}
			None => {
				assert!(true);
			}
		}
	}

	#[test]
	fn get_unit_config_by_type_id() {
		let mut unit_config = GameConfig::get_unit_config_by_type_id(1).unwrap();
		assert_eq!(unit_config.cost, GameConfig::patch_0_1_0().units[0].cost);
		assert_eq!(unit_config.hp, GameConfig::patch_0_1_0().units[0].hp);
		assert_eq!(unit_config.dmg_core, GameConfig::patch_0_1_0().units[0].dmg_core);
		assert_eq!(unit_config.dmg_unit, GameConfig::patch_0_1_0().units[0].dmg_unit);
		assert_eq!(unit_config.dmg_resource, GameConfig::patch_0_1_0().units[0].dmg_resource);
		assert_eq!(unit_config.max_range, GameConfig::patch_0_1_0().units[0].max_range);
		assert_eq!(unit_config.min_range, GameConfig::patch_0_1_0().units[0].min_range);
		assert_eq!(unit_config.speed, GameConfig::patch_0_1_0().units[0].speed);

		unit_config = GameConfig::get_unit_config_by_type_id(2).unwrap();
		assert_eq!(unit_config.cost, GameConfig::patch_0_1_0().units[1].cost);
		assert_eq!(unit_config.hp, GameConfig::patch_0_1_0().units[1].hp);
		assert_eq!(unit_config.dmg_core, GameConfig::patch_0_1_0().units[1].dmg_core);
		assert_eq!(unit_config.dmg_unit, GameConfig::patch_0_1_0().units[1].dmg_unit);
		assert_eq!(unit_config.dmg_resource, GameConfig::patch_0_1_0().units[1].dmg_resource);
		assert_eq!(unit_config.max_range, GameConfig::patch_0_1_0().units[1].max_range);
		assert_eq!(unit_config.min_range, GameConfig::patch_0_1_0().units[1].min_range);
		assert_eq!(unit_config.speed, GameConfig::patch_0_1_0().units[1].speed);

		let unit_config = GameConfig::get_unit_config_by_type_id(3);
		match unit_config {
			Some(_) => {
				assert!(false);
			}
			None => {
				assert!(true);
			}
		}
	}

	#[test]
	fn wait_till_next_tick() {

	}

	#[test]
	fn generate_u64_id() {

	}

}
