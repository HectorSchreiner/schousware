#[allow(unused_imports)]

pub mod domains;
pub mod interface;

use std::{io::BufWriter, net::TcpStream};

use ratatui::{backend::TestBackend, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::interface::app::App;

fn main() {
    println!("Hello, world!");
}
