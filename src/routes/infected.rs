use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domains::infected::{Infected, InfectedId};

pub fn get_infected_all() -> Vec<Infected> {
    todo!()
}

pub fn create_infected() {
    todo!()
}

pub fn get_infected(id: InfectedId) -> Infected {
    todo!()
}

#[derive(Error, Debug, Deserialize, Serialize, Clone, Copy)]
#[error("Command could not execute")]
pub struct CommandExecutionError;

pub fn send_command(command: crate::domains::command::Command, target: Infected) -> Result<(), CommandExecutionError> {
    todo!()
}

