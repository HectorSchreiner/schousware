use std::{net::Ipv4Addr, str::FromStr};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::domains::infected::{HostName, Infected, InfectedId, InfectedIpAddr};

pub fn get_infected_all() -> Vec<Infected> {
    todo!()
}

pub fn create_infected() -> Result<(), Error> {
    let hostname = HostName::new("Hostname".into());
    let ip: InfectedIpAddr = InfectedIpAddr::try_from("127.0.0.1".to_string())?;

    let new_infected = Infected::new(hostname, ip);
    Ok(())
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

