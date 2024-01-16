//!
//! ## Introduction
//! This module is part of the CORE Project.
//!
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//!
//! In this module the game itself gets created.
//!

use lib::game::{log::log::Logger, Game, Team};

use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let _: Logger = Logger::new();
    let listener = TcpListener::bind("127.0.0.1:4242").await.unwrap();

    let mut queue: Vec<Team> = Vec::<Team>::new();

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        queue.push(Team::from_tcp_stream(stream));

        if queue.len() >= 2 {
            let t1 = queue.remove(0);
            let t2 = queue.remove(0);
            let mut game = Game::new(vec![t1, t2]);

            tokio::spawn(async move {
                println!("Game start!");
                game.start().await;
                println!("Game ended!");
            });
        }
    }
}
