#[allow(unused_imports)]

pub mod domains;
pub mod interface;
pub mod repos;
pub mod routes;
pub mod services;

use std::{io};

use uuid;
use serde;
use crossterm::terminal;
use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::{interface::app::App, routes::infected::InfectedDatabase};

fn main() -> io::Result<()>{

    let mut terminal = ratatui::init();
    let app = App::default().run(&mut terminal);
    ratatui::restore();
    app
}
