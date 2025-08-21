use std::{fs, io::{BufReader, Read}, path::PathBuf};

use dirs::config_dir;
use ratatui::widgets::{Block, ListState};
use thiserror::Error;
use uuid::Uuid;

use crate::domains::infected::Infected;

#[derive(Error, Debug)]
pub enum InfectedDatabaseError {
    #[error("Could not find the infected configuration directory")]
    FileNotFound,
    #[error("Could not find the infected")]
    InfectedNotFound,
    #[error("unknown data store error")]
    Unknown,
    #[error(transparent)]
    Other(std::io::Error),
    #[error(transparent)]
    JsonError(serde_json::Error)
}

impl From<std::io::Error> for InfectedDatabaseError {
    fn from(value: std::io::Error) -> Self {
        Self::Other(value)
    }
}

impl From<serde_json::Error> for InfectedDatabaseError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

pub trait InfectedRepo {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError>;
    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError>;
    fn get_infected(&self, uuid: Uuid) -> Result<Infected, InfectedDatabaseError>;
    fn get_all_infected(&self) -> Result<Vec<Infected>, InfectedDatabaseError>;
}

pub struct InfectedDatabase {
    config_path: PathBuf
}

impl InfectedDatabase {
    pub fn new() -> Result<Self, InfectedDatabaseError> {
        let mut config_path = config_dir().ok_or(InfectedDatabaseError::FileNotFound)?;
        config_path.push("schousware");
        
        fs::create_dir_all(&config_path)?;
        config_path.push("infected.json");

        if !config_path.exists() {
            fs::File::create(&config_path)?;
        }
                
        Ok(Self { config_path })
    }

    fn save_infected(&self, infected_vec: &Vec<Infected>) -> Result<(), InfectedDatabaseError> {
       let data = serde_json::to_string(&infected_vec)?;
        fs::write(&self.config_path, data)?;
        Ok(())
    }
}

impl InfectedRepo for InfectedDatabase {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError> {
        let mut infected_vec = self.get_all_infected()?;
        infected_vec.push(infected.clone());
        self.save_infected(&infected_vec)
    }

    fn get_all_infected(&self) -> Result<Vec<Infected>, InfectedDatabaseError> {
        let file = fs::File::open(&self.config_path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        
        if contents.is_empty() {
            return Ok(Vec::new());
        }

        let infected_vec = serde_json::from_str(&contents)?;
        Ok(infected_vec)
    }

    fn get_infected(&self, uuid: Uuid) -> Result<Infected, InfectedDatabaseError> {
        let infected_vec = self.get_all_infected()?;

        infected_vec.into_iter()
            .find(|i| i.id.get() == uuid)
            .ok_or(InfectedDatabaseError::InfectedNotFound)
    }

    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError> {
        let infected_vec = self.get_all_infected();
        let mut new_infected_vec: Vec<Infected> = Vec::new();

        match infected_vec {
            Ok(infected_vec) => {
                for item in infected_vec.iter() {
                    if item.id.get() != uuid {
                        new_infected_vec.push(item.clone());
                    }
                }
            },
            Err(err) => return Err(InfectedDatabaseError::Unknown)
        };

        self.save_infected(&new_infected_vec).map_err(|e| InfectedDatabaseError::FileNotFound)
    }
}

