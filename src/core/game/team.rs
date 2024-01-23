use super::{bridge_con::BridgeCon, Game};

#[derive(Debug)]
pub struct Team {
    pub id: u64,
    pub start_id: u64,
    pub uuid: String,
    pub name: String,

    pub balance: u64,

    pub con: BridgeCon,
}

impl Team {
    pub fn new(start_id: u64, con: BridgeCon) -> Self {
        let id = Game::generate_u64_id();

        Team {
            id,
            start_id,
            uuid: String::from("UUID"),
            name: format!("Team {}", id),
            balance: 100,
            con,
        }
    }

    pub fn new_fake(id: u64) -> Self {
        Team {
            id,
            start_id: id,
            uuid: String::from("UUID"),
            name: format!("Team {}", id),
            balance: 100,
            con: BridgeCon::new_fake(),
        }
    }
}
