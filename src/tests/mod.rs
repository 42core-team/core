#[cfg(test)]
mod tests {
	use lib::game::{Team, Game, GameConfig};

	#[test]
	fn test_create_unit() {
		let t1 = Team::get_fake_team();
		let t2 = Team::get_fake_team() ;
		let mut game = Game::new(vec![t1, t2]);

		assert_eq!(game.teams.len(), 2);
		assert_eq!(game.units.len(), 0);
		assert_eq!(game.teams[0].balance, 100);
		assert_eq!(game.teams[1].balance, 100);
		game.create_unit(0, 1);
		assert_eq!(game.units.len(), 1);
		assert_eq!(game.teams[0].balance, 100 - GameConfig::get_unit_config_by_type_id(1).unwrap().cost);
		// Create another unit for team 0
		game.create_unit(0, 1);
		// Assertions after creating another unit
		assert_eq!(game.units.len(), 1); // Assuming that the second create_unit call fails, as suggested by your test
		assert_eq!(game.teams[0].balance, 100 - GameConfig::get_unit_config_by_type_id(1).unwrap().cost);
	}
}
