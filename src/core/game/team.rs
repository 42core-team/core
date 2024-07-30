use super::{bridge_con::BridgeCon, Game};

#[derive(Debug)]
pub struct Team {
    pub id: u64,
    pub uuid: String,
    pub name: String,

    pub balance: u64,

    pub con: BridgeCon,
}

impl Team {
    pub fn new(game: &Game, con: BridgeCon) -> Self {
        let id = Game::generate_u64_id(game);

        let cheapest_unit = game.config.units.iter().min_by_key(|unit| unit.cost);
        let cheapest_unit_cost = cheapest_unit.map_or(0, |unit| unit.cost);

        Team {
            id,
            uuid: String::from("UUID"),
            name: con.name.clone().unwrap_or_else(|| format!("Team {}", id)),
            balance: cheapest_unit_cost * 4,
            con,
        }
    }

    pub fn new_fake(id: u64) -> Self {
        Team {
            id,
            uuid: String::from("UUID"),
            name: format!("Team {}", id),
            balance: 100,
            con: BridgeCon::new_fake(),
        }
    }
}
