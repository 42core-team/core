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

fn print_character(x: u64, y: u64, state: State) {
	state.cores.iter().for_each(|core| {
		if core.x == x && core.y == y && core.team_id == 0 {
			// check for team ids for different background colors
			print!("C");
		}
	});
	state.units.iter().for_each(|unit| {
		if unit.team_id == 0 {
			if unit.x == x && unit.y == y {
				// Warrior
				if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id{
					print!("{}", "w".on_red());
				// Worker
				} else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
					print!("{}", "b".on_red());
				} else {
					println!("Unknown unit type!");
				}
			}
		} else if unit.team_id == 1 {
			if unit.x == x && unit.y == y {
				// Warrior
				if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id{
					print!("{}", "w".on_blue());
				// Worker
				} else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
					print!("{}", "b".on_blue());
				} else {
					// replace with something else since map shouldn't be compromised
					println!("Unknown unit type!");
				}
			}
		} else {
			// replace with something else since map shouldn't be compromised
			println!("Unknown team id!");
		}
	});
	state.resources.iter().for_each(|resource| {
		if resource.x == x && resource.y == y {
			print!("R");
		}
	});
	print!("D");
}

fn show_map(state: State, config: GameConfig){
	clear_map();
	let team1: u64 = state.teams[0].;
	for y in 0..config.height {
		for x in 0..config.width {
			print_character(x, y, state.clone());
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
