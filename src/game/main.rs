//!
//! ## Introduction
//! This module is part of the CORE Project.
//! 
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//! 
//! In this module the game itself gets created.
//!


use std::env;

use lib::game::Game;

#[tokio::main]
async fn main() {
	let mut reqired_team_ids = Vec::new();
	for argument in env::args() {
		println!("Argument: {}", argument);
		let n = argument.parse::<u64>();
		if n.is_ok() {
			reqired_team_ids.push(n.unwrap());
		}
    }

	let game: Game = Game::new(reqired_team_ids);
	game.init().await;
}
