use super::Game;

pub fn grant_passive_income(game: &mut Game) {
    for team in game.teams.iter_mut() {
        team.balance += game.config.idle_income;
    }
}
