use serde::{Deserialize, Serialize};

use super::{action::Travel, Game, GameConfig, Position, Vector};

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
    pub fn travel(&mut self, game: &mut Game, travel: Travel) {
        match travel.travel_type {
            Vector(vec) => {
                if vec.x == 0 && vec.y == 0 {
                    return;
                }
            }
            Position(pos) => {
                if pos.x == self.x && pos.y == self.y {
                    return;
                }
            }
        }
        self.travel = Some(travel);
    }

    pub fn update_position(&mut self, game: &mut Game) {
        match &self.travel {
            Some(pos) => {}
            Some(vec) => {}
            None => {}
        }
    }
}
