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
    pub resource_spawn_timeout: u64,
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
    pub resource_spawn_timeout: u64,
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
            resource_spawn_timeout: game_config.resource_spawn_timeout,
        }
    }
}

impl GameConfig {
    pub fn patch_0_1_0() -> Self {
        GameConfig {
            height: 10000,
            width: 10000,
            idle_income: 1,
            idle_income_timeout: 1200,
            core_hp: 25000,
            units: vec![
                UnitConfig {
                    name: String::from("Warrior"),
                    type_id: 1,
                    cost: 350,
                    hp: 3000,
                    dmg_core: 100,
                    dmg_unit: 50,
                    dmg_resource: 24,
                    max_range: 500,
                    min_range: 0,
                    speed: 35,
                },
                UnitConfig {
                    name: String::from("Worker"),
                    type_id: 2,
                    cost: 200,
                    hp: 1000,
                    dmg_core: 25,
                    dmg_unit: 15,
                    dmg_resource: 200,
                    max_range: 250,
                    min_range: 0,
                    speed: 55,
                },
                UnitConfig {
                    name: String::from("Tank"),
                    type_id: 3,
                    cost: 500,
                    hp: 15000,
                    dmg_core: 50,
                    dmg_unit: 20,
                    dmg_resource: 4,
                    max_range: 200,
                    min_range: 0,
                    speed: 15,
                },
                UnitConfig {
                    name: String::from("Archer"),
                    type_id: 4,
                    cost: 300,
                    hp: 500,
                    dmg_core: 50,
                    dmg_unit: 25,
                    dmg_resource: 24,
                    max_range: 2000,
                    min_range: 1250,
                    speed: 25,
                },
                UnitConfig {
                    name: String::from("Healer"),
                    type_id: 5,
                    cost: 350,
                    hp: 500,
                    dmg_core: -35,
                    dmg_unit: -18,
                    dmg_resource: -50,
                    max_range: 1500,
                    min_range: 750,
                    speed: 20,
                },
            ],
            teams: vec![],
            resources: vec![ResourceConfig {
                type_id: 1,
                hp: 10000,
                balance_value: 15,
            }],
            resource_spawn_timeout: 1000,
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
