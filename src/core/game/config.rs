use serde::{Deserialize, Serialize};

use super::Team;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameConfig {
    pub height: u64,
    pub width: u64,
    pub idle_income: u64,
    pub idle_income_timeout: u64,
    pub core_hp: u64,
    pub units: Vec<UnitConfig>,
    pub teams: Vec<TeamConfig>,
    pub resources: Vec<ResourceConfig>,
    pub resource_count: u64,
    pub resource_spawn_timeout: u64,
    pub unit_size: u64,

    pub flag_speed: u64,               // how fast a flying flag moves
    pub max_throw_distance: u64,       // beyond this, thrown flags fall down
    pub flag_slowdown_percentage: u64, // % slow when carrying
    pub catch_range: u64,              // how close you must be to catch lying flag
    pub jump_prepare_ticks: u64,       // ticks to wait before jump starts
    pub jump_duration_ticks: u64,      // how long jump “window” stays open
    pub jump_catch_range: u64,         // how close you must be to catch flying flag
    pub flag_capture_range: u64,       // how close to own core to score
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameConfigWithId {
    pub id: u64,
    pub height: u64,
    pub width: u64,
    pub idle_income: u64,
    pub idle_income_timeout: u64,
    pub core_hp: u64,
    pub units: Vec<UnitConfig>,
    pub teams: Vec<TeamConfig>,
    pub resources: Vec<ResourceConfig>,
    pub resource_count: u64,
    pub resource_spawn_timeout: u64,
    pub unit_size: u64,

    pub flag_speed: u64,
    pub max_throw_distance: u64,
    pub flag_slowdown_percentage: u64,
    pub catch_range: u64,
    pub jump_prepare_ticks: u64,
    pub jump_duration_ticks: u64,
    pub jump_catch_range: u64,
    pub flag_capture_range: u64,
}

impl GameConfigWithId {
    pub fn from_game_config(game_config: &GameConfig, id: u64) -> Self {
        GameConfigWithId {
            id,
            height: game_config.height,
            width: game_config.width,
            idle_income: game_config.idle_income,
            idle_income_timeout: game_config.idle_income_timeout,
            core_hp: game_config.core_hp,
            units: game_config.units.clone(),
            teams: game_config.teams.clone(),
            resources: game_config.resources.clone(),
            resource_count: game_config.resource_count,
            resource_spawn_timeout: game_config.resource_spawn_timeout,
            unit_size: game_config.unit_size,

            flag_speed: game_config.flag_speed,
            max_throw_distance: game_config.max_throw_distance,
            flag_slowdown_percentage: game_config.flag_slowdown_percentage,
            catch_range: game_config.catch_range,
            jump_prepare_ticks: game_config.jump_prepare_ticks,
            jump_duration_ticks: game_config.jump_duration_ticks,
            jump_catch_range: game_config.jump_catch_range,
            flag_capture_range: game_config.flag_capture_range,
        }
    }
}

impl GameConfig {
    pub fn patch_0_1_0() -> Self {
        GameConfig {
            height: 10000,
            width: 10000,
            idle_income: 1,
            idle_income_timeout: 3000,
            core_hp: 50000,
            resource_count: 5,
            resource_spawn_timeout: 0,
            unit_size: 200,

            flag_speed: 150,
            max_throw_distance: 2000,
            flag_slowdown_percentage: 30,
            catch_range: 50,
            jump_prepare_ticks: 5,
            jump_duration_ticks: 10,
            jump_catch_range: 75,
            flag_capture_range: 100,

            units: vec![
                UnitConfig {
                    name: String::from("Warrior"),
                    type_id: 1,
                    cost: 750,
                    hp: 3500,
                    dmg_core: 60,
                    dmg_unit: 20,
                    dmg_resource: 3,
                    max_range: 400,
                    min_range: 0,
                    speed: 30,
                },
                UnitConfig {
                    name: String::from("Worker"),
                    type_id: 2,
                    cost: 500,
                    hp: 2500,
                    dmg_core: 20,
                    dmg_unit: 7,
                    dmg_resource: 15,
                    max_range: 375,
                    min_range: 0,
                    speed: 42,
                },
                UnitConfig {
                    name: String::from("Tank"),
                    type_id: 3,
                    cost: 2000,
                    hp: 15000,
                    dmg_core: 120,
                    dmg_unit: 13,
                    dmg_resource: 23,
                    max_range: 450,
                    min_range: 0,
                    speed: 13,
                },
                UnitConfig {
                    name: String::from("Archer"),
                    type_id: 4,
                    cost: 900,
                    hp: 750,
                    dmg_core: 45,
                    dmg_unit: 25,
                    dmg_resource: 3,
                    max_range: 1400,
                    min_range: 0,
                    speed: 18,
                },
                UnitConfig {
                    name: String::from("Healer"),
                    type_id: 5,
                    cost: 1250,
                    hp: 1,
                    dmg_core: -10,
                    dmg_unit: -30,
                    dmg_resource: -2,
                    max_range: 1900,
                    min_range: 0,
                    speed: 22,
                },
            ],
            teams: vec![],
            resources: vec![ResourceConfig {
                type_id: 1,
                hp: 13500,
                balance_value: 2250,
            }],
        }
    }

    pub fn fill_team_config(config: &mut GameConfig, teams: &Vec<Team>) {
        let mut team_configs: Vec<TeamConfig> = Vec::new();
        for team in teams {
            team_configs.push(TeamConfig {
                id: team.id,
                name: team.name.clone(),
            });
        }
        config.teams = team_configs;
    }

    pub fn get_unit_config_by_type_id(&self, type_id: u64) -> Option<UnitConfig> {
        for unit in self.units.iter() {
            if unit.type_id == type_id {
                return Some(unit.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnitConfig {
    pub name: String,
    pub type_id: u64,
    pub cost: u64,
    pub hp: u64,
    pub dmg_core: i32,
    pub dmg_unit: i32,
    pub dmg_resource: i32,
    pub max_range: u64,
    pub min_range: u64,
    pub speed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamConfig {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceConfig {
    pub type_id: u64,
    pub hp: u64,
    pub balance_value: u64,
}
