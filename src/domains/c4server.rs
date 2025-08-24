use std::{net::IpAddr, path::PathBuf};
use thiserror::Error;

use crate::domains::{command::C2Command, infected::Infected};

#[derive(Debug, Clone, Default, Copy)]
pub enum C4ServerConnectionStatus {
    #[default]
    Closed,
    Open
}

#[derive(Error, Debug)]
#[error("Connection to C4 Server failed")]
pub struct C4ConnectionError;

pub struct C4Server {
    ip: IpAddr, // kan nok udsiftes med en socketaddr
    port: u16,
    pub status: C4ServerConnectionStatus
}

impl C4Server {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self { 
            ip, 
            status: C4ServerConnectionStatus::Closed,
            port,
        }
    }

    pub async fn start_listener(&mut self) -> Result<(), C4ConnectionError> {
        let addr = format!("{}:{}", self.ip, self.port).to_string();

        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => {
                self.status = C4ServerConnectionStatus::Open;
                l
            }, 
            Err(err) => {
                self.status = C4ServerConnectionStatus::Closed;
                return Err(C4ConnectionError)
            }
        };
        
        Ok(())
    }

    pub fn get_ip(&self) -> String {
        self.ip.to_string()
    }

    pub fn get_port(&self) -> String {
        self.port.to_string()
    }
}