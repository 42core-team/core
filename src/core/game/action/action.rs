use super::{Attack, Catch, Create, Jump, Throw, Travel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
    Attack(Attack),
    Create(Create),
    Travel(Travel),
    Catch(Catch),
    Throw(Throw),
    Jump(Jump),
}
