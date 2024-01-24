use serde::{Deserialize, Serialize};

use super::{Game, GameConfig};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Unit {
    pub id: u64,
    pub type_id: u64,
    pub team_id: u64,
    pub hp: u64,
    pub x: u64,
    pub y: u64,
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
                    id: Game::generate_u64_id(),
                    type_id,
                    hp: unit_config.hp,
                    x,
                    y,
                    team_id,
                });
            }
            None => return None,
        }
    }
}
