use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct Command {
    command_string: CommandString,
}

impl Command {
    fn new(&self, command_string: CommandString) -> Self {
        Self { command_string }
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