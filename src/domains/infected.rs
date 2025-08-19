use std::{net::{AddrParseError, Ipv4Addr}, str::FromStr};
use anyhow::Ok;
use serde::{Deserialize, Serialize};
use uuid::serde::braced::serialize;
use ::uuid::Uuid;
use ::thiserror::Error;

use crate::repos::database::{InfectedDatabaseError, InfectedRepo};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Infected {
    pub id: InfectedId,
    pub hostname: HostName,
    pub ip: InfectedIpAddr
}

impl Infected {
    pub fn new(hostname: HostName, ip: InfectedIpAddr) -> Self {
        Self { id: InfectedId::new(), hostname, ip }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HostName(String);

impl HostName {
    pub fn new(hostname: String) -> Self {
        Self(hostname)
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

impl From<InfectedIpAddr> for Ipv4Addr {
    fn from(infected_ip: InfectedIpAddr) -> Self {
        infected_ip.0
    }
}

impl TryFrom<String> for InfectedIpAddr {
    type Error = anyhow::Error;

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
