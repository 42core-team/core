extern crate core;

use crate::game::{Game, Team, Entity, Core, Unit, Warrior, Worker, Resource};
use lazy_static::lazy_static;

///
/// 
/// 
/// !!!!!CAUTION!!!! DEPRECATED @jgotz will change it soon
/// 
/// 
/// 




lazy_static! {
    static ref JSON_DATA: &'static str = r#"
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
    }"#;
}

/**
 * Json deserialization tests
 */

#[test]
fn json_deserialize_ok()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert_eq!(result.is_ok(), true);
}

#[test]
fn json_deserialize_map()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert_eq!(result.as_ref().unwrap().map.width, 100000);
	assert_eq!(result.as_ref().unwrap().map.height, 100000);
}

#[test]
fn json_deserialize_teams()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(result.as_ref().unwrap().map.teams.len() == 2);
	assert!(matches!(
		result.as_ref().unwrap().map.teams[0],
		Team { id: 1, name: _ }
	));
	assert!(matches!(
		result.as_ref().unwrap().map.teams[1],
		Team { id: 2, name: _ }
	));
}

#[test]
fn json_deserialize_entities()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(result.as_ref().unwrap().map.entities.len() == 7);

	assert!(matches!(
		result.as_ref().unwrap().map.entities[0],
		Entity::Core { .. }
	));
	
	assert!(matches!(
		result.as_ref().unwrap().map.entities[1],
		Entity::Core { .. }
	));

	assert!(matches!(
		result.as_ref().unwrap().map.entities[2],
		Entity::Unit { .. }
	));

	assert!(matches!(
		result.as_ref().unwrap().map.entities[3],
		Entity::Unit { .. }
	));

	assert!(matches!(
		result.as_ref().unwrap().map.entities[4],
		Entity::Unit { .. }
	));

	assert!(matches!(
		result.as_ref().unwrap().map.entities[5],
		Entity::Unit { .. }
	));

	assert!(matches!(
		result.as_ref().unwrap().map.entities[6],
		Entity::Resource { .. }
	));
}

#[test]
fn json_deserialize_core()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(matches!(
		result.as_ref().unwrap().map.entities[0],
		Entity::Core(Core { team_id: 2, x: 4, y: 5 })
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[1],
		Entity::Core(Core { team_id: 1, x: 5, y: 6 })
	));
}

#[test]
fn json_deserialize_unit()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(matches!(
		result.as_ref().unwrap().map.entities[2],
		Entity::Unit { .. }
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[3],
		Entity::Unit { .. }
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[4],
		Entity::Unit { .. }
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[5],
		Entity::Unit { .. }
	));
}

#[test]
fn json_deserialize_warrior()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(matches!(
		result.as_ref().unwrap().map.entities[2],
		Entity::Unit(Unit::Warrior(Warrior { team_id: 1, x: 5, y: 6 }))
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[3],
		Entity::Unit(Unit::Warrior(Warrior { team_id: 2, x: 6, y: 7 }))
	));
}

#[test]
fn json_deserialize_worker()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(matches!(
		result.as_ref().unwrap().map.entities[4],
		Entity::Unit(Unit::Worker(Worker { team_id: 1, x: 7, y: 6 }))
	));
	assert!(matches!(
		result.as_ref().unwrap().map.entities[5],
		Entity::Unit(Unit::Worker(Worker { team_id: 2, x: 8, y: 7 }))
	));
}

#[test]
fn json_deserialize_resource()
{
	let result: Result<Game, serde_json::Error> = serde_json::from_str(JSON_DATA.as_ref());
	assert!(matches!(
		result.as_ref().unwrap().map.entities[6],
		Entity::Resource(Resource { value: 1, x: 9, y: 6 })
	));
}