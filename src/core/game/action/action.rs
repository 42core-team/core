use super::{Attack, Create, Travel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
	Attack(Attack),
	Create(Create),
	Travel(Travel),
}
