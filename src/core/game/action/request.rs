use super::Action;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub actions: Vec<Action>,
}

impl Request {
    pub fn new(actions: Vec<Action>) -> Self {
        Self { actions }
    }
}
