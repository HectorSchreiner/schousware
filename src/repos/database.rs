use anyhow::Error;
use thiserror::Error;

use crate::domains::infected::Infected;

#[derive(Error, Debug)]
pub enum InfectedDatabaseError {
    #[error("Could not find the user's configuration directory")]
    NotFound,
}

pub trait InfectedRepo {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError>;
    fn remove_infected() -> Result<(), InfectedDatabaseError>;
    fn get_infected() -> Result<Infected, InfectedDatabaseError>;
    fn get_all_infected() -> Result<Vec<Infected>, InfectedDatabaseError>;
}

