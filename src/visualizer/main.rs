extern crate core;

use core::game::{Game, Entity};

use serde_json;

fn main() {
    let json_data = r#"
		{
			"map":{
					"width": 100000,
					"height": 100000,
					"teams": [
						{ "id": 1, "name": "Team 1" },
						{ "id": 2, "name": "Team 2" }
					],
					"entities": [
						{ "Core": { "team_id": 2, "x": 4, "y": 5 } },
						{ "Core": { "team_id": 1, "x": 5, "y": 6 } },
						{ "Unit": { "Warrior": {"team_id": 1, "x": 5, "y": 6 } } },
						{ "Unit": { "Warrior": {"team_id": 2, "x": 6, "y": 7 } } },
						{ "Unit": { "Worker": {"team_id": 1, "x": 7, "y": 6 } } },
						{ "Unit": { "Worker": {"team_id": 2, "x": 8, "y": 7 } } },
						{ "Resource": { "value": 1, "x": 9, "y": 6 } }
					]
				}
		}
    "#;

    let result: Result<Game, serde_json::Error> = serde_json::from_str(json_data);
	
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

// use std::{string, vec};
// use tokio::time::Duration;
// use tokio::runtime::Runtime;