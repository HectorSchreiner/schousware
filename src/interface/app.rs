use std::io;

use ratatui::{DefaultTerminal, Frame};

pub struct App {
    exit: ExitState
}

impl App {

    pub fn init() -> Self {
        Self { exit: ExitState::Running }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }

    fn is_running(&self) -> bool {
        match self.exit {
            ExitState::Exit => false,
            ExitState::Running => true,
        }
    }
}

pub enum ExitState {
    Running,
    Exit,
}