//!
//! ## Introduction
//! This module is part of the CORE Project.
//! 
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//! 
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!

use std::os::macos::raw::stat;

use lib::game::{bridge::bridge, Message, Login, State, GameConfig};
use tokio::net::TcpStream;


// IN THE HOME DIRECTORY 
// cargo run --manifest-path core/Cargo.toml --bin game
// cargo run --manifest-path core/Cargo.toml --bin visualizer

/// PLEASE CHANGE THIS
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:4242").await;

	if let Ok(s) = stream {
		let (sender, mut reciever, disconnect) = bridge(s);

		let config: GameConfig = GameConfig::patch_0_1_0();
		sender.send(Message::Login(Login{id: 42})).await;
		loop {
			if let Some(m) = reciever.recv().await {
				if let Message::State(x) = m {
					print(&config, x).await;
				}
			}
		}
		
	}
	
    Ok(())
}


async fn print(config: &GameConfig, state: State) {
	// DEFENETLY REMOVE THIS AT THE END PLEASE!!!!!!!
	println!("XXX {:?}", state);
	
}
