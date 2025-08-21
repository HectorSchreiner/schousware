use std::{net::IpAddr, path::PathBuf};

use file_serve::Server;

#[derive(Debug, Clone, Default, Copy)]
pub enum FileServerStatus {
    #[default]
    Closed,
    Open
}

pub struct FileServer {
    source: PathBuf,
    pub server: Server,
    pub status: FileServerStatus
}

impl FileServer {
    pub fn new(source: &PathBuf) -> Self {
        Self { source: source.to_path_buf(), server: file_serve::Server::new(&source), status: FileServerStatus::default() }
    }

    pub fn get_path(&self) -> PathBuf {
        self.source.clone()
    }

    pub fn get_addr(&self) -> &str {
        self.server.addr()
    }

    pub fn serve(&mut self) -> Result<(), file_serve::Error> {
        let result = self.server.serve();
        match &result {
            Ok(_) => self.status = FileServerStatus::Open,
            Err(_) => self.status = FileServerStatus::Closed,
        };
        result
    }

    pub fn get_status(&self) -> FileServerStatus {
        self.status
    }

    /// Close the fileserver gracefully
    pub fn close(&mut self) {
        self.server.close();
        self.status = FileServerStatus::Closed;
    }
}