use serde::{Deserialize, Serialize};

use super::GameConfig;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    pub fn from_points(start: &Position, end: &Position) -> Self {
        let x = end.x as f64 - start.x as f64;
        let y = end.y as f64 - start.y as f64;
        Vector { x, y }
    }

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

impl Position {
    pub fn new(x: u64, y: u64) -> Self {
        Position { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f64 {
        let x = self.x as f64 - other.x as f64;
        let y = self.y as f64 - other.y as f64;
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }

    pub fn is_equal(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn is_in_map(&self, config: &GameConfig) -> bool {
        self.x <= config.width && self.y <= config.height
    }
}
