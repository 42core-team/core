use serde::{Deserialize, Serialize};

use super::{entity::Unit, Core, Game, Resource, Team};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub status: u64,
    pub elapsed_ticks: u64,
    pub cores: Vec<Core>,
    pub resources: Vec<Resource>,
    pub units: Vec<Unit>,
    pub teams: Vec<StateTeam>,
}

impl State {
    pub fn from_game(game: &Game) -> Self {
        State {
            status: game.status,
            elapsed_ticks: game.elapsed_ticks,
            cores: game.cores.clone(),
            resources: game.resources.clone(),
            units: game.units.clone(),
            teams: game.teams.iter().map(|t| StateTeam::from_team(t)).collect(),
        }
    }
    pub fn from_state(state: &State) -> Self {
        State {
            status: state.status,
            elapsed_ticks: state.elapsed_ticks,
            cores: state.cores.clone(),
            resources: state.resources.clone(),
            units: state.units.clone(),
            teams: state.teams.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateTeam {
    pub id: u64,
    pub balance: u64,
}

impl StateTeam {
    fn from_team(team: &Team) -> Self {
        StateTeam {
            id: team.id,
            balance: team.balance,
        }
    }
}
