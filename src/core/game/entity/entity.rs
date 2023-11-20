//!
//! ## Introduction
//! This module handels all the different entities that are part of the Game.
//! 
//! A entitiy is either a Core or a Resource
//! 
//! A Core is the main building of a team. It can create new entities and you need to protect it.
//! 
//! A Resource is a entity that can be harvested by a Unit to get more resources.
//! 
//!

use serde::{Deserialize, Serialize};

use super::{Core, Resource};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Entity {
    Core(Core),
    Resource(Resource),
}
