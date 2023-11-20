//!
//! ## Introduction
//! This module handels all the base structs that are used in the game.
//! 
//!

pub mod game;
pub use game::Game;

pub mod unit;
pub use unit::Unit;

pub mod state;
pub use state::State;

pub mod team;
pub use team::Team;

pub mod config;
pub use config::GameConfig;
pub use config::UnitConfig;

pub mod bridge;

pub mod action;

pub mod entity;

pub mod utils;
