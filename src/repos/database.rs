use anyhow::Error;

use crate::domains::infected::Infected;

pub trait InfectedRepo {
    fn add_infected() -> Result<(), Error>;
    fn remove_infected() -> Result<(), Error>;
    fn get_infected() -> Result<Infected, Error>;
    fn get_all_infected() -> Result<Vec<Infected>, Error>;
}