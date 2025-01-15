use std::borrow::Borrow;
use std::borrow::BorrowMut;

use serde::{Deserialize, Serialize};

use crate::game::action::Travel;
use crate::game::action::TravelType::Position as PositionEnum;
use crate::game::action::TravelType::Vector as VectorEnum;
use crate::game::Game;
use crate::game::GameConfig;
use crate::game::Position;
use crate::game::UnitConfig;
use crate::game::Vector;

use super::entity_traits::EntityConfig;
use super::Entity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub team_id: u64,
    pub hp: u64,
    pub pos: Position,
    #[serde(skip)]
    travel: Option<Travel>,
    #[serde(skip)]
    pub target_id: Option<u64>,
}

impl Entity for Unit {
    fn id(&self) -> u64 {
        self.id
    }
    fn team_id(&self) -> u64 {
        self.team_id
    }
    fn pos(&self) -> &Position {
        &self.pos
    }
    fn hp(&self) -> u64 {
        self.hp
    }
    fn deal_dmg(&mut self, dmg: i32, config: &GameConfig) -> bool {
        let max_hp = config
            .units
            .iter()
            .find(|u| u.type_id == self.type_id)
            .unwrap()
            .hp;

        let mut new_hp = self.hp as i32 - dmg;
        if new_hp < 0 {
            self.hp = 0;
            return true;
        }
        if new_hp > max_hp as i32 {
            new_hp = max_hp as i32;
        }

        self.hp = new_hp as u64;
        false
    }
}

impl EntityConfig for Unit {
    fn config_dmg(&self, config: UnitConfig) -> i32 {
        return config.dmg_unit;
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
                    target_id: None,
                });
            }
            None => return None,
        }
    }

    pub fn attack(&mut self, target: impl Entity) {
        if self.id == target.id() {
            self.target_id = None;
            return;
        }

        self.target_id = Some(target.id());
    }

    pub fn calc_dmg(&self, config: &GameConfig, target: &(impl Entity + EntityConfig)) -> i32 {
        if self.target_id.is_none() {
            return 0;
        }
        let unit_config = config.get_unit_config_by_type_id(self.type_id).unwrap();

        let distance = self.pos.distance_to(target.pos());
        if distance > unit_config.max_range as f64 || distance < unit_config.min_range as f64 {
            return 0;
        }

        target.config_dmg(unit_config)
    }

    pub fn travel(&mut self, config: &GameConfig, mut travel: Travel) {
        match travel.travel_type.borrow_mut() {
            VectorEnum(vec) => {
                if vec.x == 0.0 && vec.y == 0.0 {
                    self.travel = None;
                    return;
                }
                vec.normalize();
            }
            PositionEnum(pos) => {
                if self.pos.is_equal(pos) || !self.pos.is_in_map(config) {
                    self.travel = None;
                    return;
                }
            }
        }
        self.travel = Some(travel);
    }

    pub fn update_position(&mut self, game_config: &GameConfig, units_snapshot: &Vec<Unit>) {
        if self.travel.is_none() {
            return;
        }
        // store old pos for collision checks
        let old_pos = self.pos.clone();

        let travel = self.travel.as_mut().unwrap();
        let unit_speed = GameConfig::get_unit_config_by_type_id(game_config, self.type_id);
        if unit_speed.is_none() {
            return;
        }
        let unit_speed = unit_speed.unwrap().speed;

        match travel.travel_type.borrow() {
            // -- Vector-based travel logic --
            VectorEnum(vec) => {
                if vec.x == 0.0 && vec.y == 0.0 {
                    self.travel = None;
                    return;
                }
                let new_x = self.pos.x as f64 + vec.x * unit_speed as f64;
                let new_y = self.pos.y as f64 + vec.y * unit_speed as f64;
                let new_pos = Position::new(new_x as u64, new_y as u64);

                if !new_pos.is_in_map(game_config) {
                    self.travel = None;
                    return;
                }

                // Tentatively move
                self.pos = new_pos;

                // CHANGED: check collisions
                for other_unit in units_snapshot {
                    if other_unit.id != self.id {
                        let dist_old = old_pos.distance_to(&other_unit.pos);
                        let dist_new = self.pos.distance_to(&other_unit.pos);
                        // if newly overlapping (dist < 500 now but was >= 500)
                        if dist_new < 500.0 && dist_old >= 500.0 {
                            // revert and stop traveling
                            self.pos = old_pos;
                            self.travel = None;
                            return;
                        }
                    }
                }
            }

            // -- Position-based travel logic --
            PositionEnum(pos) => {
                if self.pos.is_equal(pos) {
                    return;
                }
                let mut vec = Vector::from_points(&self.pos, pos);
                vec.normalize();

                let new_x = self.pos.x as f64 + vec.x * unit_speed as f64;
                let new_y = self.pos.y as f64 + vec.y * unit_speed as f64;
                let new_pos = Position::new(new_x as u64, new_y as u64);

                if self.pos.distance_to(&new_pos) > self.pos.distance_to(pos) {
                    self.pos = pos.clone();
                    self.travel = None;
                    return;
                }

                if !new_pos.is_in_map(game_config) {
                    self.travel = None;
                    return;
                }

                // Tentatively move
                let old_pos = self.pos.clone(); // re-store if needed
                self.pos = new_pos;

                // CHANGED: check collisions
                for other_unit in units_snapshot {
                    if other_unit.id != self.id {
                        let dist_old = old_pos.distance_to(&other_unit.pos);
                        let dist_new = self.pos.distance_to(&other_unit.pos);
                        // if newly overlapping
                        if dist_new < 500.0 && dist_old >= 500.0 {
                            // revert
                            self.pos = old_pos;
                            self.travel = None;
                            return;
                        }
                    }
                }
            }
        }
    }
}
