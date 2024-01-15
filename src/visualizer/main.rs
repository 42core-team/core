//!
//! ## Introduction
//! This module is part of the CORE Project.
//!
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//!
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!

// use std::os::macos::raw::stat;

use lib::game::{bridge::bridge, Message, Login, State, GameConfig, Game, state};
use tokio::net::TcpStream;

use crossterm::{execute, cursor, style::Stylize};
use std::io::{stdout, Write};
// use crossterm::execute::{execute, stdout};
// use crossterm::cursor;
// use std::io::Write;

// IN THE HOME DIRECTORY
// cargo run --manifest-path core/Cargo.toml --bin game
// cargo run --manifest-path Cargo.toml --bin visualizer

fn clear_map () {
	println!("{}{}", cursor::MoveTo(0, 0), cursor::Hide);
}

fn print_field(x: u64, y: u64, state: State) {
	let team1: u64 = state.teams[0].id;
	let team2: u64 = state.teams[1].id;
	state.cores.iter().for_each(|core| {
		if core.x == x && core.y == y && core.team_id == team1 {
			print!("{}", "C".on_red());
			return;
		} else if core.x == x && core.y == y && core.team_id == team2 {
			print!("{}", "C".on_blue());
			return;
		} 
		// else {
		// 	// replace with something else since map shouldn't be compromised
		// 	println!("Unknown team id!");
		// }
	});
	state.units.iter().for_each(|unit| {
		if unit.team_id == team1 {
			if unit.x == x && unit.y == y {
				// Warrior
				if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id{
					print!("{}", "w".on_red());
					return;
				// Worker
				} else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
					print!("{}", "b".on_red());
					return;
				} 
				// else {
				// 	println!("Unknown unit type!");
				// }
			}
		} else if unit.team_id == team2 {
			if unit.x == x && unit.y == y {
				// Warrior
				if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id{
					print!("{}", "w".on_blue());
					return;
				// Worker
				} else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
					print!("{}", "b".on_blue());
					return;
				} 
				// else {
				// 	// replace with something else since map shouldn't be compromised
				// 	println!("Unknown unit type!");
				// }
			}
		}
		// } else {
		// 	// replace with something else since map shouldn't be compromised
		// 	println!("Unknown team id!");
		// }
	});
	state.resources.iter().for_each(|resource| {
		if resource.x == x && resource.y == y {
			print!("{}", "R".on_white());
			return;
		}
	});
	print!("{}", " ".on_grey());
}

fn show_map(state: State, config: GameConfig){
	clear_map();
	for y in 0..config.height {
		for x in 0..config.width {
			print_field(x, y, state.clone());
		}
	}
}

// fn print_map
// test
/// PLEASE CHANGE THIS
#[tokio::main]
async fn main() -> std::io::Result<()> {
	loop {
		clear_map();
	}
	let mut stream = TcpStream::connect("127.0.0.1:4242").await;

	if let Ok(s) = stream {
		let (sender, mut reciever, disconnect) = bridge(s);
		let gameconfig: GameConfig;

		let config: GameConfig = GameConfig::patch_0_1_0(); //needs to be made dynamic after all important shit is done!!!
		sender.send(Message::Login(Login{id: 42})).await;
		if let Some(m) = reciever.recv().await {
			match m {
				Message::GameConfig(config) => {
					gameconfig = config;
				},
				_ => {
					println!("First message was not a gameconfig!");
				},
			}
		}
		loop {
			if let Some(m) = reciever.recv().await {
				match m {
					Message::State(state) => {
						show_map(state, gameconfig);
					},
					_ => {
						println!("unexpected message type!");
					},
				}
			}
		}
		
	}
	Ok(())
}

// async fn print(config: &GameConfig, state: State) {
// 	// DEFINETLY REMOVE THIS AT THE END PLEASE!!!!!!!
// 	println!("XXX {:?}", state);
	
// }
