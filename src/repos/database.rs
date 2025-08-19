use anyhow::Error;
use thiserror::Error;
use uuid::Uuid;

use crate::domains::infected::Infected;

#[derive(Error, Debug)]
pub enum InfectedDatabaseError {
    #[error("Could not find the user's configuration directory")]
    NotFound,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

impl From<anyhow::Error> for InfectedDatabaseError {
    fn from(err: anyhow::Error) -> Self {
        InfectedDatabaseError::Other(err.to_string().into())
    }
}
pub trait InfectedRepo {
    fn add_infected(&self, infected: &Infected) -> Result<(), InfectedDatabaseError>;
    fn remove_infected(&self, uuid: Uuid) -> Result<(), InfectedDatabaseError>;
    fn get_infected(&self, uuid: Uuid) -> Option<Infected>;
    fn get_all_infected(&self) -> Option<Vec<Infected>>;
}

