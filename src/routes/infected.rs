use std::{any, fs, io::{BufReader, Read, Write}, net::Ipv4Addr, path::PathBuf, str::FromStr};

use anyhow::{Error, Ok, Result};
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
    config_path: PathBuf
}

impl InfectedDatabase {
    pub fn new() -> Self {
        let mut config_path = dirs::config_dir().unwrap();       
        config_path.push("schousware");
        
        let _ = fs::create_dir_all(&config_path);
        config_path.push("infected.json");

        let _ = fs::File::create(&config_path);
                
        Self { config_path }
    }

    fn save_infected(&self, infected_vec: &Vec<Infected>) -> Result<(), InfectedDatabaseError> {
       let data = serde_json::to_string(&infected_vec)
            .map_err(|e| InfectedDatabaseError::Other(e.into()))?;

        fs::write(&self.config_path, data)
            .map_err(|e| InfectedDatabaseError::Other(e.into()))
        
    }
}

impl InfectedRepo for InfectedDatabase {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError> {
        let infected_vec = self.get_all_infected();

        if let anyhow::Result::Ok(mut vec) = infected_vec {
            vec.push(infected.clone());
            return self.save_infected(&vec);
        } else {
            return Err(InfectedDatabaseError::FileNotFound);
        }        
    }

    fn get_all_infected(&self) -> Result<Vec<Infected>, InfectedDatabaseError> {
        let file = fs::File::open(self.config_path.clone());
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut contents = String::new();
        let _ = buf_reader.read_to_string(&mut contents).map_err(|_| InfectedDatabaseError::FileNotFound);

        let infected_vec: Result<Vec<Infected>, InfectedDatabaseError> = serde_json::from_str(&contents).map_err(|e| InfectedDatabaseError::FileNotFound);
        infected_vec
    }

    fn get_infected(&self, uuid: Uuid) -> Result<Infected, InfectedDatabaseError> {
        let infected_vec = self.get_all_infected();

        match infected_vec {
            anyhow::Result::Ok(vec) => {
                match vec.iter().find(|i| i.id.get() == uuid).cloned() {
                    Some(infected) => anyhow::Result::Ok(infected),
                    None => Err(InfectedDatabaseError::InfectedNotFound)
                }
            },
            anyhow::Result::Err(err) => {
                Err(InfectedDatabaseError::Other(err.into()))
            }
        }
    }

    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError> {
        let infected_vec = self.get_all_infected();
        let mut new_infected_vec: Vec<Infected> = Vec::new();

        match infected_vec {
            anyhow::Result::Ok(infected_vec) => {
                for item in infected_vec.iter() {
                    if item.id.get() != uuid {
                        new_infected_vec.push(item.clone());
                    }
                }
            },
            anyhow::Result::Err(err) => return Err(InfectedDatabaseError::Other(err.into()))
        };

        self.save_infected(&new_infected_vec).map_err(|e| InfectedDatabaseError::FileNotFound)
    }
}

