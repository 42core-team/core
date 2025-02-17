#[cfg(test)]
mod tests {

    use std::time::Duration;

    use lib::game::{
        bridge::bridge, helper::Target, log::log, Core, Game, GameConfig, Message, Position, Team,
    };
    use tokio::{io::AsyncWriteExt, net::TcpStream, select, sync::oneshot, time::timeout};

    fn get_fake_game() -> Game {
        let mut game = Game::new(vec![1, 2]);
        game.time_since_last_tick = game.tick_rate;
        game.teams = vec![Team::new_fake(1), Team::new_fake(2)];
        game.cores = vec![
            Core::new(
                &game,
                1,
                Position::new(2000, 2000),
                GameConfig::patch_0_1_0().core_hp,
            ),
            Core::new(
                &game,
                2,
                Position::new(4000, 4000),
                GameConfig::patch_0_1_0().core_hp,
            ),
        ];
        game
    }

    fn get_fake_game_without_teams() -> Game {
        let game = Game::new(vec![1, 2]);
        game
    }

    #[test]
    ///
    /// Test if the fake team creation works
    ///
    /// The fake team is used to test the game logic
    ///
    fn test_create_fake_team() {
        let team = Team::new_fake(1);
        assert_eq!(team.id, 1);
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
        assert_eq!(game.teams[0].id, 1);
        assert_eq!(game.teams[1].id, 2);
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
        game.create_unit(game.teams[0].id, 1);
        assert_eq!(game.units.len(), 1);
        assert_eq!(
            game.teams[0].balance,
            100 - GameConfig::get_unit_config_by_type_id(&game.config, 1)
                .unwrap()
                .cost
        );
        // Create another unit for team 1
        game.create_unit(1, 1);
        // second create_unit call fails -> balance to low
        assert_eq!(game.units.len(), 1);
        // balance should not change
        assert_eq!(
            game.teams[0].balance,
            100 - GameConfig::get_unit_config_by_type_id(&game.config, 1)
                .unwrap()
                .cost
        );
        //same for second team
        game.create_unit(2, 2);
        assert_eq!(game.units.len(), 2);
        assert_eq!(game.teams[1].balance, 50);
        game.create_unit(2, 2);
        assert_eq!(game.units.len(), 3);
        assert_eq!(game.teams[1].balance, 0);
        game.create_unit(2, 2);
        assert_eq!(game.units.len(), 3);
        assert_eq!(game.teams[1].balance, 0);
    }

    #[test]
    ///
    /// Test for invalid input in the create_unit function
    ///
    fn test_invalid_input_create_unit() {
        let mut game = get_fake_game();

        assert_eq!(game.teams.len(), 2);
        assert_eq!(game.units.len(), 0);
        assert_eq!(game.teams[0].balance, 100);
        assert_eq!(game.teams[1].balance, 100);

        // invalid team id
        game.create_unit(3, 1);
        assert_eq!(game.units.len(), 0);
        assert_eq!(game.teams[0].balance, 100);
        assert_eq!(game.teams[1].balance, 100);

        // invalid unit type id
        game.create_unit(1, 3);
        assert_eq!(game.units.len(), 0);
        assert_eq!(game.teams[0].balance, 100);
        assert_eq!(game.teams[1].balance, 100);

        // invalid team id and unit type id
        game.create_unit(3, 3);
        assert_eq!(game.units.len(), 0);
        assert_eq!(game.teams[0].balance, 100);
        assert_eq!(game.teams[1].balance, 100);

        // invalid team id and valid unit type id
        game.create_unit(3, 1);
        assert_eq!(game.units.len(), 0);
        assert_eq!(game.teams[0].balance, 100);
        assert_eq!(game.teams[1].balance, 100);

        // valid team id and invalid unit type id
        game.create_unit(1, 3);
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

        let core1 = game.get_core_by_team_id(1);
        assert_eq!(core1.unwrap().pos.x, 2000);
        assert_eq!(core1.unwrap().pos.y, 2000);

        let core2 = game.get_core_by_team_id(2);
        assert_eq!(core2.unwrap().pos.x, 4000);
        assert_eq!(core2.unwrap().pos.y, 4000);

        let core3 = game.get_core_by_team_id(3);
        assert_eq!(core3, None);
    }

    #[test]
    fn test_get_team_by_id() {
        let game = get_fake_game();

        let team1 = game.get_team_by_id(1);
        match team1 {
            Some(team) => {
                assert_eq!(team.name, "Team 1");
            }
            None => {
                assert!(false);
            }
        }

        let team2 = game.get_team_by_id(2);
        match team2 {
            Some(team) => {
                assert_eq!(team.name, "Team 2");
            }
            None => {
                assert!(false);
            }
        }

        let team3 = game.get_team_by_id(3);
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

        let team1 = game.get_team_by_id_mut(1);
        assert_eq!(team1.unwrap().name, "Team 1");

        let team2 = game.get_team_by_id_mut(2);
        assert_eq!(team2.unwrap().name, "Team 2");

        let team3 = game.get_team_by_id_mut(3);
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
        let mut unit_config =
            GameConfig::get_unit_config_by_type_id(&GameConfig::patch_0_1_0(), 1).unwrap();
        assert_eq!(unit_config.cost, GameConfig::patch_0_1_0().units[0].cost);
        assert_eq!(unit_config.hp, GameConfig::patch_0_1_0().units[0].hp);
        assert_eq!(
            unit_config.dmg_core,
            GameConfig::patch_0_1_0().units[0].dmg_core
        );
        assert_eq!(
            unit_config.dmg_unit,
            GameConfig::patch_0_1_0().units[0].dmg_unit
        );
        assert_eq!(
            unit_config.dmg_resource,
            GameConfig::patch_0_1_0().units[0].dmg_resource
        );
        assert_eq!(
            unit_config.max_range,
            GameConfig::patch_0_1_0().units[0].max_range
        );
        assert_eq!(
            unit_config.min_range,
            GameConfig::patch_0_1_0().units[0].min_range
        );
        assert_eq!(unit_config.speed, GameConfig::patch_0_1_0().units[0].speed);

        unit_config =
            GameConfig::get_unit_config_by_type_id(&GameConfig::patch_0_1_0(), 2).unwrap();
        assert_eq!(unit_config.cost, GameConfig::patch_0_1_0().units[1].cost);
        assert_eq!(unit_config.hp, GameConfig::patch_0_1_0().units[1].hp);
        assert_eq!(
            unit_config.dmg_core,
            GameConfig::patch_0_1_0().units[1].dmg_core
        );
        assert_eq!(
            unit_config.dmg_unit,
            GameConfig::patch_0_1_0().units[1].dmg_unit
        );
        assert_eq!(
            unit_config.dmg_resource,
            GameConfig::patch_0_1_0().units[1].dmg_resource
        );
        assert_eq!(
            unit_config.max_range,
            GameConfig::patch_0_1_0().units[1].max_range
        );
        assert_eq!(
            unit_config.min_range,
            GameConfig::patch_0_1_0().units[1].min_range
        );
        assert_eq!(unit_config.speed, GameConfig::patch_0_1_0().units[1].speed);

        let unit_config = GameConfig::get_unit_config_by_type_id(&GameConfig::patch_0_1_0(), 3);
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
    ///
    /// Generate 10000 ids and check that they are unique
    ///
    fn generate_u64_id() {
        let game = get_fake_game();
        let mut ids: Vec<u64> = Vec::new();
        for _ in 0..10000 {
            let id = Game::generate_u64_id(&game);
            assert!(!ids.contains(&id));
            ids.push(id);
        }
    }

    #[test]
    fn get_target_by_id() {
        let mut game = get_fake_game();
        game.create_unit(0, 1);
        for unit in game.units.iter() {
            let target = game.get_target_by_id(unit.id);
            match target {
                Some(Target::Unit(_)) => {
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }

    #[test]
    ///
    /// Units:
    /// 0: (2000, 2000)
    /// 1: (9000, 9000)
    /// 2: (2100, 2100)
    /// 3: (8000, 8000)
    ///
    /// Resources:
    /// 0: (5000, 5000)
    ///
    /// Cores:
    /// 0: (2000, 2000)
    /// 1: (4000, 4000)
    ///
    /// Actual Distances:
    /// 0 -> 1: 9899
    /// 0 -> 2: 141
    /// 0 -> 3: 8485
    ///
    /// 0 -> r: 4242
    /// 0 -> c1: 0
    /// 0 -> c2: 2828
    ///
    /// 1 -> 2: 9758
    /// 1 -> 3: 1414
    ///
    /// 1 -> r: 5656
    /// 1 -> c1: 9899
    /// 1 -> c2: 7071
    ///
    /// 2 -> 3: 8343
    ///
    /// 2 -> r: 4101
    /// 2 -> c1: 141
    /// 2 -> c2: 2687
    ///
    /// 3 -> r: 4242
    /// 3 -> c1: 8485
    /// 3 -> c2: 5656
    ///
    /// Ranges:
    /// 0: 1000
    /// 1: 1000
    /// 2: 200
    /// 3: 200
    ///
    /// Result:
    /// 0 -> 1: false
    /// 0 -> 2: true
    /// 0 -> 3: false
    /// 0 -> r: false
    /// 0 -> c1: true
    /// 0 -> c2: false
    /// 1 -> 2: false
    /// 1 -> 3: false
    /// 1 -> r: false
    /// 1 -> c1: false
    /// 1 -> c2: false
    /// 2 -> 3: false
    /// 2 -> r: false
    /// 2 -> c1: true
    /// 2 -> c2: false
    /// 3 -> r: false
    /// 3 -> c1: false
    /// 3 -> c2: false
    ///
    fn is_target_in_range() {
        let mut game = get_fake_game();
        game.create_fake_resource(Position::new(5000, 5000));
        game.create_fake_unit(1, 1, Position::new(2000, 2000));
        game.create_fake_unit(1, 2, Position::new(9000, 9000));
        game.create_fake_unit(2, 1, Position::new(2100, 2100));
        game.create_fake_unit(2, 2, Position::new(8000, 8000));

        let unit1 = game.units[0].clone();
        let unit2 = game.units[1].clone();
        let unit3 = game.units[2].clone();
        let unit4 = game.units[3].clone();

        let unit_id1 = unit1.id;
        let unit_id2 = unit2.id;
        let unit_id3 = unit3.id;
        let unit_id4 = unit4.id;

        let _u1 = game.get_target_by_id(unit_id1);
        let u2 = game.get_target_by_id(unit_id2);
        let u3 = game.get_target_by_id(unit_id3);
        let u4 = game.get_target_by_id(unit_id4);
        let r = game.get_target_by_id(game.resources[0].id);
        let c1 = game.get_target_by_id(game.cores[0].id);
        let c2 = game.get_target_by_id(game.cores[1].id);

        // assert!(!game.is_target_in_range(unit1.id, &u2));
        // assert!(game.is_target_in_range(unit1.id, &u3));
        // assert!(!game.is_target_in_range(unit1.id, &u4));
        // assert!(!game.is_target_in_range(unit1.id, &r));
        // assert!(game.is_target_in_range(unit1.id, &c1));
        // assert!(!game.is_target_in_range(unit1.id, &c2));
        // assert!(!game.is_target_in_range(unit2.id, &u3));
        // assert!(!game.is_target_in_range(unit2.id, &u4));
        // assert!(!game.is_target_in_range(unit2.id, &r));
        // assert!(!game.is_target_in_range(unit2.id, &c1));
        // assert!(!game.is_target_in_range(unit2.id, &c2));
        // assert!(!game.is_target_in_range(unit3.id, &u4));
        // assert!(!game.is_target_in_range(unit3.id, &r));
        // assert!(game.is_target_in_range(unit3.id, &c1));
        // assert!(!game.is_target_in_range(unit3.id, &c2));
        // assert!(!game.is_target_in_range(unit4.id, &r));
        // assert!(!game.is_target_in_range(unit4.id, &c1));
        // assert!(!game.is_target_in_range(unit4.id, &c2));
    }

    #[test]
    ///
    /// Units:
    /// 0: (2000, 2000)
    /// 1: (9000, 9000)
    /// 2: (2100, 2100)
    /// 3: (8000, 8000)
    ///
    /// Resources:
    /// 0: (5000, 5000)
    ///
    /// Cores:
    /// 0: (2000, 2000)
    /// 1: (4000, 4000)
    ///
    /// Actual Distances:
    /// 0 -> 1: 9899
    /// 0 -> 2: 141
    /// 0 -> 3: 8485
    ///
    /// 0 -> r: 4242
    /// 0 -> c1: 0
    /// 0 -> c2: 2828
    ///
    /// 1 -> 2: 9758
    /// 1 -> 3: 1414
    ///
    /// 1 -> r: 5656
    /// 1 -> c1: 9899
    /// 1 -> c2: 7071
    ///
    /// 2 -> 3: 8343
    ///
    /// 2 -> r: 4101
    /// 2 -> c1: 141
    /// 2 -> c2: 2687
    ///
    /// 3 -> r: 4242
    /// 3 -> c1: 8485
    /// 3 -> c2: 5656
    ///
    /// Ranges:
    /// 0: 1000
    /// 1: 1000
    /// 2: 200
    /// 3: 200
    ///
    /// Result:
    /// 0 -> 1: false
    /// 0 -> 2: true
    /// 0 -> 3: false
    /// 0 -> r: false
    /// 0 -> c1: true
    /// 0 -> c2: false
    /// 1 -> 2: false
    /// 1 -> 3: false
    /// 1 -> r: false
    /// 1 -> c1: false
    /// 1 -> c2: false
    /// 2 -> 3: false
    /// 2 -> r: false
    /// 2 -> c1: true
    /// 2 -> c2: false
    /// 3 -> r: false
    /// 3 -> c1: false
    /// 3 -> c2: false
    ///
    fn attack() {
        let mut game = get_fake_game();

        game.create_fake_resource(Position::new(5000, 5000));
        game.create_fake_unit(1, 1, Position::new(2000, 2000));
        game.create_fake_unit(1, 2, Position::new(9000, 9000));
        game.create_fake_unit(2, 1, Position::new(2100, 2100));
        game.create_fake_unit(2, 2, Position::new(8000, 8000));

        let unit1 = game.units[0].clone();
        let unit2 = game.units[1].clone();
        let unit3 = game.units[2].clone();
        let unit4 = game.units[3].clone();

        let unit_id1 = unit1.id;
        let unit_id2 = unit2.id;
        let unit_id3 = unit3.id;
        let unit_id4 = unit4.id;

        let r_id = game.resources[0].id;
        let c1_id = game.cores[0].id;
        let c2_id = game.cores[1].id;

        let mut before_hp = game.units[1].hp;
        // 0 -> 1: false
        // game.attack(unit_id1, unit_id2);
        // hp of unit2 should not change -> be the same as in the GameConfig
        // assert_eq!(game.units[1].hp, before_hp);
        // 0 -> 2: true
        before_hp = game.units[2].hp;
        // game.attack(unit_id1, unit_id3);
        // hp of unit3 should change -> be lower than in the GameConfig
        log::info(&format!(
            "unit3 hp: {}",
            game.get_unit_by_id(unit_id3).unwrap().hp
        ));
        // assert!(game.get_unit_by_id(unit_id3).unwrap().hp < before_hp);
        // 0 -> 3: false
        before_hp = game.units[3].hp;
        // game.attack(unit_id1, unit_id4);
        // hp of unit4 should not change -> be the same as in the GameConfig
        // assert_eq!(game.units[3].hp, before_hp);
        // 0 -> r: false
        before_hp = game.resources[0].hp;
        // game.attack(unit_id1, r_id);
        // hp of resource should not change -> be the same as in the GameConfig
        assert!(game.resources[0].hp == before_hp);
        // 0 -> c1: true
        before_hp = game.cores[0].hp;
        // game.attack(unit_id1, c1_id);
        // hp of core1 should change -> be lower than in the GameConfig
        // assert!(game.cores[0].hp < before_hp);
        // 0 -> c2: false
        before_hp = game.cores[1].hp;
        // game.attack(unit_id1, c2_id);
        // hp of core2 should not change -> be the same as in the GameConfig
        assert_eq!(game.cores[1].hp, before_hp);
        // 1 -> 2: false
        before_hp = game.units[2].hp;
        // game.attack(unit_id2, unit_id3);
        // hp of unit3 should not change -> be the same as in the GameConfig
        assert_eq!(game.units[2].hp, before_hp);
        // 1 -> 3: false
        // game.attack(unit_id2, unit_id4);
        // hp of unit4 should not change -> be the same as in the GameConfig
        assert_eq!(
            game.units[3].hp,
            GameConfig::get_unit_config_by_type_id(&game.config, unit4.type_id)
                .unwrap()
                .hp
        );
        // 1 -> r: false
        // game.attack(unit_id2, r_id);
        // hp of resource should not change -> be the same as in the GameConfig
        // assert_eq!(game.resources[0].hp, game.resources[0].hp);
        // 1 -> c1: false
        before_hp = game.cores[0].hp;
        // game.attack(unit_id2, c1_id);
        // hp of core1 should not change -> be the same as in the GameConfig
        // assert_eq!(game.cores[0].hp, before_hp);
        // 1 -> c2: false
        before_hp = game.cores[1].hp;
        // game.attack(unit_id2, c2_id);
        // hp of core2 should not change -> be the same as in the GameConfig
        // assert_eq!(game.cores[1].hp, before_hp);
        // 2 -> 3: false
        before_hp = game.units[3].hp;
        // game.attack(unit_id3, unit_id4);
        // hp of unit4 should not change -> be the same as in the GameConfig
        // assert_eq!(game.units[3].hp, before_hp);
        // 2 -> r: false
        before_hp = game.resources[0].hp;
        // game.attack(unit_id3, r_id);
        // hp of resource should not change -> be the same as in the GameConfig
        // assert_eq!(game.resources[0].hp, before_hp);
        // 2 -> c1: true
        before_hp = game.cores[0].hp;
        // game.attack(unit_id3, c1_id);
        // hp of core1 should change -> be lower than in the GameConfig
        // assert!(game.cores[0].hp < before_hp);
        // 2 -> c2: false
        // game.attack(unit_id3, c2_id);
        // hp of core2 should not change -> be the same as in the GameConfig
        // assert_eq!(game.cores[1].hp, GameConfig::patch_0_1_0().core_hp);
        // 3 -> r: false
        before_hp = game.resources[0].hp;
        // game.attack(unit_id4, r_id);
        // hp of resource should not change -> be the same as in the GameConfig
        // assert_eq!(game.resources[0].hp, before_hp);
        // 3 -> c1: false
        before_hp = game.cores[0].hp;
        // game.attack(unit_id4, c1_id);
        // hp of core1 should not change -> be the same as in the GameConfig
        // assert_eq!(game.cores[0].hp, before_hp);
        // 3 -> c2: false
        before_hp = game.cores[1].hp;
        // game.attack(unit_id4, c2_id);
        // hp of core2 should not change -> be the same as in the GameConfig
        // assert_eq!(game.cores[1].hp, before_hp);
    }

    #[tokio::test]
    async fn game_login_config() {
        let (tx1, rx1) = oneshot::channel::<()>();
        let (tx2, rx2) = oneshot::channel::<()>();
        let (tx3, rx3) = oneshot::channel::<()>();
        let (tx4, rx4) = oneshot::channel::<()>();
        let mut _tick_rate: u64 = 50;

        let game = get_fake_game_without_teams();
        tokio::spawn(async move {
            game.init().await;
        });

        tokio::spawn(async move {
            let mut stream;
            loop {
                stream = TcpStream::connect("127.0.0.1:4242").await;
                if stream.is_ok() {
                    break;
                }
            }
            let mut stream = stream.unwrap();

            stream.write("{\"id\": 1}".as_bytes()).await.unwrap();

            let (_sender, mut receiver, _disconnect) = bridge(stream);

            let mut result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(false);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(true);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(true);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(false);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            let _ = tx1.send(());
        });

        // Try to connect with an invalid id
        tokio::spawn(async move {
            let mut stream;
            loop {
                stream = TcpStream::connect("127.0.0.1:4242").await;
                if stream.is_ok() {
                    break;
                }
            }
            let mut stream = stream.unwrap();

            stream.write("{\"id\": 3}".as_bytes()).await.unwrap();

            let (_sender, mut receiver, _disconnect) = bridge(stream);

            let result = timeout(Duration::from_millis(100), receiver.recv()).await;
            assert!(!result.is_ok());
            let _ = tx3.send(());
        });

        // Spawn the second secondary thread
        tokio::spawn(async move {
            let mut stream;
            loop {
                stream = TcpStream::connect("127.0.0.1:4242").await;
                if stream.is_ok() {
                    break;
                }
            }
            let mut stream = stream.unwrap();

            stream.write("{\"id\": 2}".as_bytes()).await.unwrap();

            let (_sender, mut receiver, _disconnect) = bridge(stream);

            let mut result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(false);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(true);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(true);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(false);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            let _ = tx2.send(());
        });

        // Try to connect as spectator
        tokio::spawn(async move {
            let mut stream;
            loop {
                stream = TcpStream::connect("127.0.0.1:4242").await;
                if stream.is_ok() {
                    break;
                }
            }
            let mut stream = stream.unwrap();

            stream.write("{\"id\": 42}".as_bytes()).await.unwrap();

            let (_sender, mut receiver, _disconnect) = bridge(stream);

            let mut result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(false);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(true);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            result = receiver.recv().await;
            assert!(result.is_some());
            match result {
                Some(message) => match message {
                    Message::State(_) => {
                        assert!(true);
                    }
                    Message::GameConfigWithId(_) => {
                        assert!(false);
                    }
                    Message::VecAction(_) => {
                        assert!(false);
                    }
                    Message::Login(_) => {
                        assert!(false);
                    }
                },
                None => {
                    assert!(false);
                }
            }
            let _ = tx4.send(());
        });

        select! {
            _ = rx1 => {}
            _ = rx2 => {}
            _ = rx3 => {}
            _ = rx4 => {}
        }
    }
}
