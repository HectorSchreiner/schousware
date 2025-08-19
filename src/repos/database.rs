use std::error;

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

