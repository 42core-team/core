use serde::{Deserialize, Serialize};

use crate::game::{Position, Vector};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TravelType {
    Vector(Vector),
    Position(Position),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Travel {
    pub id: u64,
    pub travel_type: TravelType,
}
