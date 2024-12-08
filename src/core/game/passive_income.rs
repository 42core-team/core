use super::{Game, GameConfig};

fn get_income_per_tick(game_config: &GameConfig, time_since_last_tick: u128) -> u64 {
    let income_per_second = game_config.idle_income;
    let income_per_tick = income_per_second * time_since_last_tick as u64 / 1000;
    income_per_tick
}

pub fn grant_passive_income(game: &mut Game) {
    let income_per_tick = get_income_per_tick(&game.config, game.time_since_last_tick);

    for team in game.teams.iter_mut() {
        team.balance += income_per_tick;
    }
}
