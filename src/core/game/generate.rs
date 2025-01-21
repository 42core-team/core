use std::{cmp, f64::consts::PI};

use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{Core, Game, Position, Resource};

pub fn cores(game: &Game) -> Vec<Core> {
    let team_count = game.teams.len();
    let mut cores: Vec<Core> = Vec::new();

    if team_count == 2 {
        cores.push(Core::new(
            game,
            game.teams[0].id,
            Position::new(0, 0),
            game.config.core_hp,
        ));
        cores.push(Core::new(
            game,
            game.teams[1].id,
            Position::new(game.config.width, game.config.height),
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
                Position::new(x as u64, y as u64),
                game.config.core_hp,
            ));
        }
    }
    cores
}

fn rnd_pos(game: &mut Game) -> (Position, Position) {
    let mut rng = StdRng::seed_from_u64(game.seed + game.resource_counter);

    let pos1 = Position::new(
        rng.gen_range(0..game.config.width / 2),
        rng.gen_range(0..game.config.height),
    );

    let pos2 = Position::new(game.config.width - pos1.x, game.config.height - pos1.y);

    game.resource_counter += 1;
    (pos1, pos2)
}

pub fn resources(game: &mut Game) -> Vec<Resource> {
    let mut resources: Vec<Resource> = Vec::new();

    for _ in 0..game.config.width * game.config.height / 10000000 {
        let (pos1, pos2) = rnd_pos(game);
        let resource_config = &game.config.resources[0];

        resources.push(Resource::new(
            game,
            resource_config.type_id,
            pos1,
            resource_config.hp,
        ));
        resources.push(Resource::new(
            game,
            resource_config.type_id,
            pos2,
            resource_config.hp,
        ));
    }

    resources
}

pub fn spawn_new_resources(game: &mut Game) -> () {
    if game.elapsed_ticks >= game.config.resource_spawn_timeout {
        return;
    }

    let resource_count: u64 = game.resources.len() as u64;

    if resource_count < game.config.resource_count {
        let (pos1, pos2) = rnd_pos(game);
        let resource_config = &game.config.resources[0];

        game.resources.push(Resource::new(
            game,
            resource_config.type_id,
            pos1,
            resource_config.hp,
        ));
        game.resources.push(Resource::new(
            game,
            resource_config.type_id,
            pos2,
            resource_config.hp,
        ));
    }
}
