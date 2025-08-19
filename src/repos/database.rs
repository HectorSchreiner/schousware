use thiserror::Error;
use uuid::Uuid;

use crate::domains::infected::Infected;

#[derive(Error, Debug)]
pub enum InfectedDatabaseError {
    #[error("Could not find the infected configuration directory")]
    FileNotFound,
    #[error("Could not find the infected")]
    InfectedNotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub trait InfectedRepo {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError>;
    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError>;
    fn get_infected(&self, uuid: Uuid) -> Result<Infected, InfectedDatabaseError>;
    fn get_all_infected(&self) -> Result<Vec<Infected>, InfectedDatabaseError>;
}

