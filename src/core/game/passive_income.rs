use super::Game;

pub fn grant_passive_income(game: &mut Game) {
    if game.elapsed_ticks >= game.config.idle_income_timeout {
        return;
    }

    for team in game.teams.iter_mut() {
        team.balance += game.config.idle_income;
    }
}
