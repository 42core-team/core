use std::time::Duration;

use super::{utils::get_ms, Resource, Core, GameConfig, State, Team, Unit};

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

            for team in self.teams.iter_mut() {
                team.update();
            }

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
        let min_ms_per_tick = 3000;

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
}
