#[allow(unused_imports)]

pub mod domains;
pub mod interface;

use std::{io};

use crossterm::terminal;
use ratatui::{backend::TestBackend, buffer::Buffer, layout::Rect, prelude::{Backend, CrosstermBackend}, Terminal};

use crate::interface::app::App;

fn main() -> io::Result<()>{
    let mut terminal = ratatui::init();
    let app = App::default().run(&mut terminal);
    ratatui::restore();
    println!("{:?}", app);
    app
}
