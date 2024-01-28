use std::borrow::Borrow;
use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};

use super::{action::Travel, Game, GameConfig};
use crate::game::action::TravelType::Position;
use crate::game::action::TravelType::Vector;
use crate::game::log::log;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub team_id: u64,
    pub hp: u64,
    pub x: u64,
    pub y: u64,
    #[serde(skip)]
    travel: Option<Travel>,
}

///
/// Unit implementation
///
impl Unit {
    ///
    /// Function to create a new unit
    ///
    pub fn new(game: &mut Game, team_id: u64, type_id: u64, x: u64, y: u64) -> Option<Self> {
        let unit_config = GameConfig::get_unit_config_by_type_id(&game.config, type_id);
        let team = game.get_team_by_id(team_id);
        if team.is_none() {
            return None;
        }
        match unit_config {
            Some(unit_config) => {
                return Some(Unit {
                    id: Game::generate_u64_id(game),
                    type_id,
                    hp: unit_config.hp,
                    x,
                    y,
                    team_id,
                    travel: None,
                });
            }
            None => return None,
        }
    }

    /**
     * Give the travel command to the unit
     */
    pub fn travel(&mut self, mut travel: Travel) {
        match travel.travel_type.borrow_mut() {
            Vector(vec) => {
                if vec.x == 0 && vec.y == 0 {
                    self.travel = None;
                    return;
                }
                let vec_magnitude = ((vec.x.pow(2) + vec.y.pow(2)) as f64).sqrt();
                vec.x = (vec.x as f64 * 10.0 / vec_magnitude) as i64;
                vec.y = (vec.y as f64 * 10.0 / vec_magnitude) as i64;
            }
            Position(pos) => {
                if pos.x == self.x && pos.y == self.y {
                    self.travel = None;
                    return;
                }
            }
        }
        self.travel = Some(travel);
    }

    pub fn update_position(&mut self, time_since_last_tick: u128, game_config: &GameConfig) {
        if self.travel.is_none() {
            return;
        }
        let travel = self.travel.as_mut().unwrap();
        let unit_speed = GameConfig::get_unit_config_by_type_id(game_config, self.type_id);
        if unit_speed.is_none() {
            return;
        }
        let unit_speed = unit_speed.unwrap().speed;

        match travel.travel_type.borrow() {
            Vector(vec) => {
                let new_x =
                    self.x as i64 + vec.x * time_since_last_tick as i64 * unit_speed as i64 / 1000;
                let new_y =
                    self.y as i64 + vec.y * time_since_last_tick as i64 * unit_speed as i64 / 1000;
                if new_x >= 0
                    && new_y >= 0
                    && new_x <= game_config.width as i64
                    && new_y <= game_config.height as i64
                {
                    log::info(&format!("Unit {} moved to {}, {}", self.id, new_x, new_y));
                    self.x = new_x as u64;
                    self.y = new_y as u64;
                } else {
                    self.travel = None;
                }
            }
            Position(pos) => {
                if pos.x == self.x && pos.y == self.y {
                    return;
                }
                self.x = pos.x;
                self.y = pos.y;
            }
        }
    }
}
