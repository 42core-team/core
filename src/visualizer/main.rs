//!
//! ## Introduction
//! This module is part of the CORE Project.
//! 
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//! 
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!


use lib::game::bridge::bridge;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
	let mut stream;
	loop {
		stream = TcpStream::connect("127.0.0.1:4242").await;
		if stream.is_ok() {
			break;
		}
	}
	
	let stream = stream.unwrap();
	let (_sender, mut receiver, _disconnect) = bridge(stream);
	println!("Connected to server!");
	
	loop {
		let message = receiver.recv().await;
		match message {
			Some(actions) => {
				println!("Actions: {:?}", actions);
			},
			None => {
				println!("Connection closed by server");
				break;
			}
		}
	}
}
