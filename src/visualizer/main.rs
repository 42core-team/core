extern crate core;

use core::game::{Game, Entity};
use core::socket_client::{connect, get_response};

use serde_json;
use std::error::Error;

fn remove_after_last_brace(input: &str) -> String {
    let reversed_string: String = input.chars().rev().collect();

    if let Some(index) = reversed_string.find('}') {
        let truncated_string = &reversed_string[index..].chars().rev().collect::<String>();
        truncated_string.to_string()
    } else {
        input.to_string()
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut socket = connect("127.0.0.1:4242").await?;

	loop {
		let json_data = remove_after_last_brace(get_response(&mut socket).await?.trim());

		println!("Received data: {}", json_data.as_str().trim_end_matches(" \n\t"));

		let result: Result<Game, serde_json::Error> = serde_json::from_str(json_data.as_str().trim_end_matches(" \n\t"));

		println!("---------- Game data: ---------\nWidth: {:?}\nHeight: {:?}\n",
			result.as_ref().unwrap().map.width,
			result.as_ref().unwrap().map.height
		);

		println!("---------- Teams: ---------");
		if result.as_ref().unwrap().map.teams.len() == 0 {
			println!("No teams");
		}
		else {
			for team in result.as_ref().unwrap().map.teams.iter() {
				println!("{:?}", team);
			}
		}
		println!("");

		println!("---------- Entities: ---------");
		match result {
			Ok(item) => {
				for e in item.map.entities {
					match e {
						Entity::Core(core) => {
							println!("{:?}", core);
						}
						Entity::Unit(unit) =>{
							match unit {
								core::game::Unit::Warrior(warrior) => {
									println!("{:?}", warrior);
								},
								core::game::Unit::Worker(worker) => {
									println!("{:?}", worker);
								},
							}
						},
						Entity::Resource(resource) => {
							println!("{:?}", resource);
						},
					}
				}

			}
			Err(e) => {
				println!("Error parsing JSON: {:?}", e);
			}
		}
	}
}

// use std::{string, vec};
// use tokio::time::Duration;
// use tokio::runtime::Runtime;
