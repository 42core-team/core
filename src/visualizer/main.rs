//!
//! ## Introduction
//! This module is part of the CORE Project.
//!
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//!
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!

use lib::game::{bridge::bridge, state, Game, GameConfig, Login, Message, State};
use tokio::net::TcpStream;

use crossterm::{cursor, execute, style::Stylize};
use std::io::{stdout, Write};

// IN THE HOME DIRECTORY
// cargo run --manifest-path core/Cargo.toml --bin game
// cargo run --manifest-path Cargo.toml --bin visualizer OR make visualizer

const SCALE: u64 = 1000;

fn get_coordinates(x: u64, y: u64) -> (u64, u64) {
    let x = x / SCALE;
    let y = y / SCALE;
    (x, y)
}

fn next_1000(n: u64) -> u64 {
    ((n + (SCALE - 1)) / SCALE) * SCALE
}

fn print_field(x: u64, y: u64, state: State) {
    let team1: u64 = state.teams[0].id;
    let team2: u64 = state.teams[1].id;
    let mut coord: (u64, u64) = get_coordinates(x, y);

    for core in &state.cores {
        coord = get_coordinates(core.x, core.y);
        if (coord.0 == x && coord.1 == y) && core.team_id == team1 {
            print!("{}", "C".on_red());
            return;
        } else if (coord.0 == x && coord.1 == y) && core.team_id == team2 {
            print!("{}", "C".on_blue());
            return;
        }
    }
    for unit in &state.units {
        coord = get_coordinates(unit.x, unit.y);
        if (coord.0 == x && coord.1 == y) && unit.team_id == team1 {
            // Warrior
            if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id {
                print!("{}", "w".on_red());
                return;
            // Worker
            } else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
                print!("{}", "b".on_red());
                return;
            }
        } else if (coord.0 == x && coord.1 == y) && unit.team_id == team2 {
            // Warrior
            if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id {
                print!("{}", "w".on_blue());
                return;
            // Worker
            } else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
                print!("{}", "b".on_blue());
                return;
            }
        }
    }
    for resource in &state.resources {
        coord = get_coordinates(resource.x, resource.y);
        if coord.0 == x && coord.1 == y {
            print!("{}", "R".on_white());
            return;
        }
    }
}

fn show_map(state: State, width: u64, height: u64) {
    let mut y: u64 = 0;
    let mut x: u64;
    while y < height {
        x = 0;
        while x < width {
            print_field(x, y, state.clone());
            if x == width - 1 {
                print!("\n");
            }
        }
    }
}

fn print_field(x: &mut u64, y: &mut u64, state: State) {
    let team1: u64 = state.teams[0].id;
    let team2: u64 = state.teams[1].id;

    for core in &state.cores {
        if core.x == *x && core.y == *y && core.team_id == team1 {
            print!("{}", "C".on_red());
            *x = next_1000(*x);
            *y = next_1000(*y);
            return;
        } else if core.x == *x && core.y == *y && core.team_id == team2 {
            print!("{}", "C".on_blue());
            *x = next_1000(*x);
            *y = next_1000(*y);
            return;
        }
    }

    for unit in &state.units {
        if unit.team_id == team1 {
            if unit.x == *x && unit.y == *y {
                // Warrior
                if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id {
                    print!("{}", "w".on_red());
                    *x = next_1000(*x);
                    *y = next_1000(*y);
                    return;
                // Worker
                } else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
                    print!("{}", "b".on_red());
                    *x = next_1000(*x);
                    *y = next_1000(*y);
                    return;
                }
            }
        } else if unit.team_id == team2 {
            if unit.x == *x && unit.y == *y {
                // Warrior
                if unit.type_id == GameConfig::patch_0_1_0().units[0].type_id {
                    print!("{}", "w".on_blue());
                    *x = next_1000(*x);
                    *y = next_1000(*y);
                    return;
                // Worker
                } else if unit.type_id == GameConfig::patch_0_1_0().units[1].type_id {
                    print!("{}", "b".on_blue());
                    *x = next_1000(*x);
                    *y = next_1000(*y);
                    return;
                }
            }
        }
    }

    for resource in &state.resources {
        if resource.x == *x && resource.y == *y {
            print!("{}", "R".on_white());
            *x = next_1000(*x);
            *y = next_1000(*y);
            return;
        }
    }
    if *x == SCALE - 1 && *y == SCALE - 1 {
        print!("{}", " ".on_grey());
    }
    // *x += 1;
    // *y += 1;
}

fn show_map(state: State, config: GameConfig) {
    print!("{}{}", cursor::MoveTo(0, 0), cursor::Hide);
    let mut y: u64 = 0;
    let mut x: u64;
    let mut old_x: u64;
    let mut old_y: u64;
    while y < config.height {
        old_y = y;
        x = 0;
        while x < config.width {
            old_x = x;
            print_field(&mut x, &mut y, state.clone());
            if x == config.width / SCALE && y == config.height / SCALE {
                print!("\n");
            }
            if x == old_x {
                x += 1;
            }
        }
        if y == old_y {
            y += 1;
        }
    }
}

// fn print_map
// test
/// PLEASE CHANGE THIS
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:4242").await;

    if let Ok(s) = stream {
        let (sender, mut reciever, _disconnect) = bridge(s);
        let mut game_config: GameConfig = GameConfig::patch_0_1_0();

        let _config: GameConfig = GameConfig::patch_0_1_0(); //needs to be made dynamic after all important shit is done!!!
        let _ = sender.send(Message::Login(Login { id: 42 })).await;
        if let Some(m) = reciever.recv().await {
            match m {
                Message::GameConfig(_config) => {
                    game_config = _config;
                }
                _ => {
                    println!("First message was not a gameconfig!");
                }
            }
        }
        const SCALE: u64 = 1000;
        let WIDTH: u64 = game_config.width;
        let HEIGHT: u64 = game_config.height;
        loop {
            if let Some(m) = reciever.recv().await {
                match m {
                    Message::State(state) => {
                        show_map(state, WIDTH, HEIGHT);
                    }
                    _ => {
                        println!("unexpected message type!");
                    }
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
