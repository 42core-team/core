use std::borrow::Borrow;
use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};

use crate::game::action::Travel;
use crate::game::action::TravelType::Position as PositionEnum;
use crate::game::action::TravelType::Vector as VectorEnum;
use crate::game::Game;
use crate::game::GameConfig;
use crate::game::Position;
use crate::game::Vector;

use super::Entity;
use super::EntityTeam;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub team_id: u64,
    pub hp: u64,
    pub pos: Position,
    #[serde(skip)]
    travel: Option<Travel>,
}

impl Entity for Unit {
    fn id(&self) -> u64 {
        self.id
    }
    fn pos(&self) -> &Position {
        &self.pos
    }
    fn hp(&self) -> u64 {
        self.hp
    }
}

impl EntityTeam for Unit {
    fn team_id(&self) -> u64 {
        self.team_id
    }
}

impl Unit {
    pub fn new(game: &mut Game, team_id: u64, type_id: u64, pos: Position) -> Option<Self> {
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
                    pos,
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
            VectorEnum(vec) => {
                if vec.x == 0.0 && vec.y == 0.0 {
                    self.travel = None;
                    return;
                }
                vec.normalize();
            }
            PositionEnum(pos) => {
                if self.pos.is_equal(pos) {
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
            VectorEnum(vec) => {
                let new_x = self.pos.x as f64
                    + vec.x * time_since_last_tick as f64 * unit_speed as f64 / 1000.0;
                let new_y = self.pos.y as f64
                    + vec.y * time_since_last_tick as f64 * unit_speed as f64 / 1000.0;
                let new_pos = Position::new(new_x as u64, new_y as u64);

                if !new_pos.is_in_map(game_config) {
                    self.travel = None;
                    return;
                }
                self.pos = new_pos;
            }
            PositionEnum(pos) => {
                if self.pos.is_equal(pos) {
                    return;
                }
                let mut vec = Vector::from_points(&self.pos, pos);
                vec.normalize();

                let new_x = self.pos.x as f64
                    + vec.x * time_since_last_tick as f64 * unit_speed as f64 / 1000.0;
                let new_y = self.pos.y as f64
                    + vec.y * time_since_last_tick as f64 * unit_speed as f64 / 1000.0;
                let new_pos = Position::new(new_x as u64, new_y as u64);

                if !new_pos.is_in_map(game_config) {
                    self.travel = None;
                    return;
                }

                if self.pos.distance_to(&new_pos) > self.pos.distance_to(pos) {
                    self.pos = pos.clone();
                    self.travel = None;
                    return;
                }
                self.pos = new_pos;
            }
        }
    }
}
