use super::{Attack, Create, Travel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Action {
	Attack(Attack),
	Create(Create),
	Travel(Travel),
}

impl Action {
	pub fn new_attack(attack: Attack) -> Self {
		Self::Attack(attack)
	}

	pub fn new_create(create: Create) -> Self {
		Self::Create(create)
	}

	pub fn new_travel(travel: Travel) -> Self {
		Self::Travel(travel)
	}
}