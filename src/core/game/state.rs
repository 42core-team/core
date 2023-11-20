use serde::{Deserialize, Serialize};

use super::{Game, Team, Unit, entity::{Resource, Core}};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub status: u8,
    pub resources: Vec<Resource>,
	pub cores: Vec<Core>,
    pub units: Vec<Unit>,
    pub teams: Vec<StateTeam>,
}

impl State {
    pub fn from_game(game: &Game) -> Self {
        State {
            status: 0, // @TODO THAT STUFF
            resources: game.resources.clone(),
			cores: game.cores.clone(),
            units: game.units.clone(),
            teams: game.teams.iter().map(|t| StateTeam::from_team(t)).collect(),
        }
    }
    pub fn from_state(state: &State) -> Self {
        State {
            status: state.status,
            resources: state.resources.clone(),
			cores: state.cores.clone(),
            units: state.units.clone(),
            teams: state.teams.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateTeam {
    id: u64,
    balance: u64,
}

impl StateTeam {
    fn from_team(team: &Team) -> Self {
        StateTeam {
            id: team.id,
            balance: team.balance,
        }
    }
}
