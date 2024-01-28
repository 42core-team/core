use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn normalize(&mut self) {
        let vec_magnitude = (self.x.powf(2.0) + self.y.powf(2.0)).sqrt();
        self.x = self.x as f64 / vec_magnitude;
        self.y = self.y as f64 / vec_magnitude;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Position {
    pub x: u64,
    pub y: u64,
}
