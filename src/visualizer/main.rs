//!
//! ## Introduction
//! This module is part of the CORE Project.
//!
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//!
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!

// use lib::game::{bridge::bridge, state, Game, GameConfig, Login, Message, State};
use crossterm::style::Stylize;
use lib::game::{bridge::bridge, GameConfig, Login, Message, State};
use tokio::net::TcpStream;

const SCALE: u64 = 1000;

fn get_coordinates(x: u64, y: u64) -> (u64, u64) {
    let x = x / SCALE;
    let y = y / SCALE;
    (x, y)
}

fn print_field(x: u64, y: u64, state: State) {
    // let team1: u64 = state.teams[0].id;
    // let team2: u64 = state.teams[1].id;
    let team1: u64 = state.cores[0].team_id;
    let team2: u64 = state.cores[1].team_id;
    let mut coord: (u64, u64);

    for core in &state.cores {
        // debug
        coord = get_coordinates(core.pos.x, core.pos.y);
        if (coord.0 == x && coord.1 == y) && core.team_id == team1 {
            print!("{}", "C".white().on_red());
            return;
        } else if (coord.0 == x && coord.1 == y) && core.team_id == team2 {
            print!("{}", "C".white().on_blue());
            return;
        }
        // coord = get_coordinates(core.x, core.y);
        // if coord.0 == x && coord.1 == y {
        //     print!("{}", "C".white().on_red());
        //     return;
        // }
    }
    let warrior: u64 = GameConfig::patch_0_1_0().units[0].type_id;
    let worker: u64 = GameConfig::patch_0_1_0().units[1].type_id;
    for unit in &state.units {
        coord = get_coordinates(unit.pos.x, unit.pos.y);
        if (coord.0 == x && coord.1 == y) && unit.team_id == team1 {
            if unit.type_id == warrior {
                print!("{}", "w".white().on_red());
                return;
            } else if unit.type_id == worker {
                // b for builder since w is already taken
                print!("{}", "b".white().on_red());
                return;
            }
        } else if (coord.0 == x && coord.1 == y) && unit.team_id == team2 {
            if unit.type_id == warrior {
                print!("{}", "w".white().on_blue());
                return;
            } else if unit.type_id == worker {
                print!("{}", "b".white().on_blue());
                return;
            }
            // !!!
            // add shit here when new units released!
            // !!!
        }
    }
    // !!!
    // edit this as soon as ressource-ids are introduced!
    // vvvvvvvvvvvvvvvv
    for resource in &state.resources {
        coord = get_coordinates(resource.pos.x, resource.pos.y);
        // if coord.0 == x && coord.1 == y && resource.type_id == 1 {
        if coord.0 == x && coord.1 == y {
            print!("{}", "R".white().on_black());
            return;
        }
        // else if coord.0 == x && coord.1 == y && resource.type_id == 2 {
        // 	print!("{}", "R".on_white());
        // 	return;
        // }
    }
    // ^^^^^^^^^^^^^^^
    // edit this as soon as ressource-ids are introduced!
    // !!!
    print!("{}", " ".on_grey());
    // possible changes: black bg, border with specific chars
    // print!("{}", " ");
}

fn show_map(state: State, width: u64, height: u64) {
    print!("\x1B[2J\x1B[1;1H");
    for y in 0..height {
        for x in 0..width {
            print_field(x, y, state.clone());
            if x == width - 1 {
                println!("");
            }
        }
    }
}

/// steps for testing
/// 4 terminals
/// terminal 1: make game
/// terminal 2: make visualizer
/// terminal 3:
/// 	nc localhost 4242
/// 	{"id": 10}
/// terminal 4:
/// 	nc localhost 4242
/// 	{"id": 20}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:4242").await;

    if let Ok(s) = stream {
        let (sender, mut reciever, _disconnect) = bridge(s);
        let mut game_config: GameConfig = GameConfig::patch_0_1_0(); //needs to be made dynamic after all important shit is done!!!
        let _ = sender.send(Message::Login(Login { id: 42 })).await;
        if let Some(m) = reciever.recv().await {
            match m {
                Message::GameConfig(_config) => {
                    game_config = _config;
                    println!("gameconfig recieved");
                }
                _ => {
                    println!("First message was not a gameconfig!");
                }
            }
        } else {
            println!("error");
        }
        const SCALE: u64 = 1000;
        let width: u64 = game_config.width / SCALE;
        let height: u64 = game_config.height / SCALE;

        loop {
            if let Some(m) = reciever.recv().await {
                match m {
                    Message::State(state) => {
                        show_map(state, width, height);
                    }
                    _ => {
                        println!("unexpected message type!");
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}
