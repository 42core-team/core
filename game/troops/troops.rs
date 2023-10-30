use std;
use interface;

pub fn create_troop(name: String, troop_type: Troop_Type)
{
	let troop = Troop {
		name: this.name,
		troop_type: this.troop_type,
		health: 100,
		dps: 5,
		position: Vector2 {
			// get the core pos
		},
		team = 0,
		action_function: || {
			println!("Action performed!");
		}
	};
	//push troop to a list of all troops
}

pub fn delete_troop()
{
	//pop the troop out of the list
}

pub fn move_troop()
{
	// performs the move action for that specific troop by id
}
