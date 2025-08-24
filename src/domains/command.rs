use std::process::Command;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct C2Command(std::process::Command);

impl C2Command {
    fn new(&self, command_string: CommandString) -> Self {
        Self(Command::new(command_string.0))
    } 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandString(String);

#[derive(Error, Debug, Clone, Copy)]
#[error("Not a valid command")]
pub struct CommandCreationError;

impl CommandString {
    pub fn new(&self, command: String) -> Result<Self, CommandCreationError> {
        Ok(Self(command))
    }
}