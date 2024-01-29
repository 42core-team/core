//!
//! ## Introduction
//! This module is part of the CORE Project.
//!
//! The Project ist a coding challenge to create a bot that can play the game CORE.
//!
//! In this module you will find the basic visualizer that connects to the game and visualizes the game state.
//!

use lib::game::action::{Action, Create};

fn main() {
    let a: Action = lib::game::action::Action::Create(Create { type_id: 4 });

    let mut aa: Vec<Action> = vec![];
    aa.push(a);
    let json_string = serde_json::to_string(&aa).unwrap();
    println!("{}", json_string);
}
