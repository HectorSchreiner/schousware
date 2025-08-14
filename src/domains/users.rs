use serde::{Deserialize, Serialize};
use ::uuid::Uuid;
use ::thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct User {
    username: Username,
    id: UserId
}

#[derive(Deserialize, Serialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Self {
        Username(username)
    }
}

#[derive(Error, Debug)]
pub enum NewUserError {
    #[error("username: `{0}` aldready exists")]
    NameAlreadyExists(String),
    #[error("username cannot be empty")]
    EmtpyUsername
}

impl User {
    pub fn new(username: Username) -> Result<Self, NewUserError> {
        if username.0.is_empty() {
            return Err(NewUserError::EmtpyUsername);
        }
        
        Ok(Self { username, id: UserId::new() })
    }
}

#[derive(Deserialize, Serialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}
