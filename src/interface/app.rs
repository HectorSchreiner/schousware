use std::{default, io};


use anyhow::Error;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    menu: MenuState,
    exit: ExitState
}

impl App {

    pub fn init() -> Self {
        Self { menu: MenuState::default(), exit: ExitState::default() }
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
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {},
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = ExitState::Exit,
            KeyCode::Char('1') => self.menu = MenuState::MainMenu,
            KeyCode::Char('2') => self.menu = MenuState::UserMenu,
            _ => return
        }2
    }

    fn is_running(&self) -> bool {
        match self.exit {
            ExitState::Exit => false,
            ExitState::Running => true,
        }
    }

    fn render_main_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        let layout = Layout::default()
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),

        ]).split(area);

        let title = Line::from(" Schousware C3 Server ");
        let title2 = Line::from(" Users ");

        let instructions = Line::from(vec![
            " Exit ".into(),
            "<ESC> ".into()
            ]);

        let instructions2 = Line::from(vec![
            " Up ".into(),
            " <Up Arrow> ".into()
            ]);

        let main = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::ROUNDED)
        .render(layout[0], buffer);

         let users = Block::bordered()
        .title(title2.centered())
        .title_bottom(instructions2.centered())
        .border_set(border::ROUNDED)
        .render(layout[1], buffer);   
    }

    fn render_user_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        let title = Line::from(" User Menu ");

        let instructions = Line::from(vec![
            " Exit ".into(),
            "<ESC> ".into()
            ]);

        let main = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::ROUNDED)
        .render(area, buffer);
    }
}

#[derive(Debug, Default)]
pub enum ExitState {
    #[default]
    Running,
    Exit,
}

#[derive(Debug, Default)]
pub enum MenuState {
    #[default]
    MainMenu,
    UserMenu,
}
impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buffer: &mut Buffer)
        where
            Self: Sized 
    {
        match self.menu {
            MenuState::MainMenu => self.render_main_menu(area, buffer),
            MenuState::UserMenu => self.render_user_menu(area, buffer),
        }
    }
}