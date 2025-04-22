use crate::game::{GameConfig, Position};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlagState {
    Lying,   // on the ground
    Carried, // held by a unit
    Flying,  // mid‑air after a throw
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flag {
    pub state: FlagState,
    pub pos: Position,                // current location (if lying or mid‑flight)
    pub carrier_id: Option<u64>,      // who’s carrying, if any
    pub target_pos: Option<Position>, // where it’s headed when flying
}

impl Flag {
    pub fn new_center(config: &GameConfig) -> Self {
        let x = config.width / 2;
        let y = config.height / 2;
        Flag {
            state: FlagState::Lying,
            pos: Position::new(x, y),
            carrier_id: None,
            target_pos: None,
        }
    }
}
