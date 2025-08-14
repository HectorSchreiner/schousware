use ip::IpAddr;
use serde::{Deserialize, Serialize};
use uuid::serde::braced::serialize;
use ::uuid::Uuid;
use ::thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Infected {
    id: InfectedId,
    hostname: HostName,
    ip: InfectedIpAddr
}

impl Infected {
    pub fn new(hostname: HostName, ip: InfectedIpAddr) -> Self {
        Self { id: InfectedId::new(), hostname, ip }
    }
}

#[derive(Deserialize, Serialize)]
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


#[derive(Deserialize, Serialize)]
pub struct InfectedId(Uuid);

impl InfectedId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn get(&self) -> Uuid {
        self.0
    }
}
#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct InfectedIpAddr(IpAddr);

impl From<IpAddr> for InfectedIpAddr {
    fn from(value: IpAddr) -> Self {
        Self(value)
    }
}

impl From<InfectedIpAddr> for IpAddr {
    fn from(infected_ip: InfectedIpAddr) -> Self {
        infected_ip.0
    }
}

impl TryFrom<String> for InfectedIpAddr {
    type Error = <IpAddr as std::str::FromStr>::Err;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse::<IpAddr>().map(InfectedIpAddr)
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

