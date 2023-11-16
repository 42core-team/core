extern crate core;

use core::game::{Update, Entity};
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

		//println!("Received data: {}", json_data.as_str().trim_end_matches(" \n\t"));

		let result: Result<Update, serde_json::Error> = serde_json::from_str(json_data.as_str().trim_end_matches(" \n\t"));

		match result {
			Ok(update) => {
				println!("---------- Entities: ---------");
				for entity in update.entities {
					match entity {
						Entity::Core(core) => {
							println!("Core id: {} x {} y {} hp {} team id {}", core.id, core.x, core.y, core.hp, core.team_id);
						},
						Entity::Resource(resource) => {
							println!("Resource id: {} value {} x {} y {}", resource.id, resource.value, resource.x, resource.y);
						}
					}
				}
				for unit in update.units {
					println!("Unit id: {} type id {} hp {} x {} y {} team id {}", unit.id, unit.type_id, unit.hp, unit.x, unit.y, unit.team_id);
				}
			},
			Err(error) => {
				println!("Error parsing json {}", error);
				continue;
			}
		}
	}
}

// use std::{string, vec};
// use tokio::time::Duration;
// use tokio::runtime::Runtime;
