use std::process::Command;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct C4Command(String);

impl C4Command {
    fn new(&self, command: String) -> Self {
        Self(command)
    } 

    pub fn get_as_str(&self) -> &str {
        &self.0
    }
}