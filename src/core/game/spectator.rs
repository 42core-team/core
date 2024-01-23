use super::bridge_con::BridgeCon;

#[derive(Debug)]
pub struct Spectator {
    pub con: BridgeCon,
}

impl Spectator {
    pub fn new(con: BridgeCon) -> Self {
        Spectator { con }
    }
}
