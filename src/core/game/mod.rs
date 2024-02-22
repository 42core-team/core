//!
//! ## Introduction
//! This module handels all the base structs that are used in the game.
//!
//!

pub mod game;
pub use game::Game;

pub mod entity;
pub use entity::Core;
pub use entity::Entity;
pub use entity::EntityTeam;
pub use entity::Resource;
pub use entity::Unit;

pub mod state;
pub use state::State;

pub mod team;
pub use team::Team;

pub mod config;
pub use config::GameConfig;
pub use config::UnitConfig;

pub mod message;
pub use message::Message;

pub mod spectator;
pub use spectator::Spectator;

pub mod login;
pub use login::Login;

pub mod bridge;

pub mod action;

pub mod helper;

pub mod utils;

pub mod log;

pub mod bridge_con;

pub mod generate;

pub mod passive_income;

pub mod position;
pub use position::Position;
pub use position::Vector;
