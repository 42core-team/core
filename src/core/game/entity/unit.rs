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

    pub fn update_position(&mut self, config: &GameConfig, others: &[&Unit]) {
        if self.travel.is_some() {
            let travel = self.travel.as_mut().unwrap();
            let unit_speed = GameConfig::get_unit_config_by_type_id(config, self.type_id);
            if unit_speed.is_none() {
                return;
            }
            let unit_speed = unit_speed.unwrap().speed;

            match travel.travel_type.borrow() {
                VectorEnum(vec) => {
                    let raw_x = self.pos.x as f64 + vec.x * unit_speed as f64;
                    let raw_y = self.pos.y as f64 + vec.y * unit_speed as f64;
                    let clamped_x = clamp_to_map(raw_x, config.width as f64);
                    let clamped_y = clamp_to_map(raw_y, config.height as f64);
                    let new_pos = Position::new(clamped_x as u64, clamped_y as u64);

                    if !new_pos.is_in_map(config) {
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

                    let raw_x = self.pos.x as f64 + vec.x * unit_speed as f64;
                    let raw_y = self.pos.y as f64 + vec.y * unit_speed as f64;
                    let clamped_x = clamp_to_map(raw_x, config.width as f64);
                    let clamped_y = clamp_to_map(raw_y, config.height as f64);
                    let new_pos = Position::new(clamped_x as u64, clamped_y as u64);

                    if self.pos.distance_to(&new_pos) > self.pos.distance_to(pos) {
                        self.pos = pos.clone();
                        self.travel = None;
                        return;
                    }

                    if !new_pos.is_in_map(config) {
                        self.travel = None;
                        return;
                    }
                    self.pos = new_pos;
                }
            }
        }
        self.resolve_collisions(others, config);
    }

    fn resolve_collisions(&mut self, others: &[&Unit], config: &GameConfig) {
        let collision_radius: f64 = config.unit_size as f64;

        let mut push_vector = Vector::new(0.0, 0.0);

        for other in others {
            if other.id == self.id {
                continue;
            }

            let dx = (self.pos.x as f64) - (other.pos.x as f64);
            let dy = (self.pos.y as f64) - (other.pos.y as f64);

            let distance = self.pos.distance_to(&other.pos);
            let min_dist = collision_radius * 2.0;

            if distance < min_dist {
                if distance > 0.000001 {
                    let nx = dx / distance;
                    let ny = dy / distance;
                    let overlap = min_dist - distance;
                    push_vector.x += nx * overlap;
                    push_vector.y += ny * overlap;
                } else {
                    // random push so units exactly inside each other can move
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let random_angle = rng.gen_range(0.0..(2.0 * std::f64::consts::PI));

                    let overlap = min_dist / 2.0;
                    let nx = random_angle.cos();
                    let ny = random_angle.sin();
                    push_vector.x += nx * overlap;
                    push_vector.y += ny * overlap;
                }
            }
        }

        // apply push
        let length = (push_vector.x * push_vector.x + push_vector.y * push_vector.y).sqrt();
        if length > 0.0 {
            let slide_factor = 0.5;
            push_vector.x *= slide_factor;
            push_vector.y *= slide_factor;

            self.pos.x =
                clamp_to_map(self.pos.x as f64 + push_vector.x, config.width as f64) as u64;

            self.pos.y =
                clamp_to_map(self.pos.y as f64 + push_vector.y, config.height as f64) as u64;
        }
    }
}

/// Helper that ensures we remain in-bounds 🍆🤏
fn clamp_to_map(value: f64, max_value: f64) -> f64 {
    if value < 0.0 {
        0.0
    } else if value > max_value {
        max_value
    } else {
        value
    }
}
