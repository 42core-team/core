use std::{cmp, f64::consts::PI};

use rand::Rng;

use super::{Core, Game, Resource};

pub fn cores(game: &Game) -> Vec<Core> {
    let team_count = game.teams.len();
    let mut cores: Vec<Core> = Vec::new();

    if team_count == 2 {
        cores.push(Core::new(game, game.teams[0].id, 0, 0, game.config.core_hp));
        cores.push(Core::new(
            game,
            game.teams[1].id,
            game.config.width,
            game.config.height,
            game.config.core_hp,
        ));
    } else if team_count > 2 {
        let radius = cmp::min(game.config.width, game.config.height) as f64 / 2.0;
        let center_x = game.config.width as f64 / 2.0;
        let center_y = game.config.height as f64 / 2.0;
        let angle_step = 2.0 * PI / team_count as f64;

        for (i, team) in game.teams.iter().enumerate() {
            let angle = angle_step * i as f64;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            cores.push(Core::new(
                game,
                team.id,
                x as u64,
                y as u64,
                game.config.core_hp,
            ));
        }
    }
    cores
}

pub fn resources(game: &Game) -> Vec<Resource> {
    let mut resources: Vec<Resource> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..game.config.width * game.config.height / 10000000 {
        let x = rng.gen_range(0..game.config.width);
        let y = rng.gen_range(0..game.config.height);

        resources.push(Resource::new(game, 1, 0, x, y, 500));
    }

    resources
}
