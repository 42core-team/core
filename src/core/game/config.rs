use serde::{Deserialize, Serialize};

use super::Team;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameConfig {
    pub height: u64,
    pub width: u64,
    pub idle_income: u64,
    pub core_hp: u64,
    pub units: Vec<UnitConfig>,
    pub teams: Vec<TeamConfig>,
}

impl GameConfig {
    pub fn patch_0_1_0() -> Self {
        GameConfig {
            height: 10000,
            width: 10000,
            idle_income: 25,
            core_hp: 5000,
            units: vec![
                UnitConfig {
                    name: String::from("Warrior"),
                    type_id: 1,
                    cost: 100,
                    hp: 3000,
                    dmg_core: 2000,
                    dmg_unit: 1500,
                    dmg_resource: 500,
                    max_range: 1000,
                    min_range: 0,
                    speed: 10,
                },
                UnitConfig {
                    name: String::from("Worker"),
                    type_id: 2,
                    cost: 50,
                    hp: 1500,
                    dmg_core: 500,
                    dmg_unit: 250,
                    dmg_resource: 2000,
                    max_range: 200,
                    min_range: 0,
                    speed: 20,
                },
            ],
            teams: vec![],
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

    pub fn get_unit_config_by_type_id(config: &GameConfig, type_id: u64) -> Option<UnitConfig> {
        for unit in config.units.iter() {
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
    pub dmg_core: u64,
    pub dmg_unit: u64,
    pub dmg_resource: u64,
    pub max_range: u64,
    pub min_range: u64,
    pub speed: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TeamConfig {
    pub id: u64,
    pub name: String,
}
