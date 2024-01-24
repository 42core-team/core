use super::{Attack, Create, Travel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
    Attack(Attack),
    Create(Create),
    TravelTo(Travel),
    TravelDir(Travel),
}

impl Action {
    pub fn new_attack(attack: Attack) -> Self {
        Self::Attack(attack)
    }

    pub fn new_create(create: Create) -> Self {
        Self::Create(create)
    }

    pub fn new_travel_to(travel_to: Travel) -> Self {
        Self::TravelTo(travel_to)
    }

    pub fn new_travel_dir(travel_dir: Travel) -> Self {
        Self::TravelDir(travel_dir)
    }
}
