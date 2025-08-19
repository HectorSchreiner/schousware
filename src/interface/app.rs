use std::{default, io, net::{IpAddr, Ipv4Addr}, str::FromStr, vec};

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

use crate::{domains::{self, infected::{self, HostName, Infected, InfectedIpAddr}}, repos::database::{self, InfectedRepo}, routes::infected::InfectedDatabase};

#[derive(Debug, Default)]
pub struct App {
    menu: MenuState,
    exit: ExitState,
    infected_machines: Vec<Infected>,
}

impl App {
    pub fn init() -> Self {
        let infected_machines: Vec<Infected> = Vec::new();

        Self { menu: MenuState::default(), exit: ExitState::default(), infected_machines }
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
            KeyCode::Char('3') => self.menu = MenuState::InfectedMenu,
            KeyCode::Char('4') => self.menu = MenuState::StatsMenu,
            _ => {}
        }
    }

    fn is_running(&self) -> bool {
        match self.exit {
            ExitState::Exit => false,
            ExitState::Running => true,
        }
    }

    fn menu_selection(&self) -> Line<'static> {
        Line::from(" <1>Main <2>Users <3>Infected <4>Stats")
    } 

    fn menu_instructions(&self) -> Line<'static> {
        Line::from(vec![
            " Exit ".into(),
            "<ESC>".into(),
            " Change Menu ".into(),
            "<NUM> ".into()
            ])
    }

    fn default_menu_instruction(&self, title: &'static str, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        let title = Line::from(title);

        return Block::bordered()
        .title(title.centered())
        .title_top(self.menu_instructions().left_aligned())
        .title_bottom(self.menu_selection().left_aligned())
        .border_set(border::ROUNDED)
        .render(area, buffer)
    }

    fn render_main_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Main ", area, buffer)        
    }

    fn render_user_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Users ", area, buffer);

    }

    fn render_infected_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Infected ", area, buffer);
        let db = InfectedDatabase::new();
        let infected = Infected::new(HostName::new("hostname"), InfectedIpAddr::from_str("127.0.0.1").unwrap());
        db.add_infected(&infected).unwrap();
    }

    fn render_stats_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Stats ", area, buffer);
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
    InfectedMenu,
    StatsMenu,
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buffer: &mut Buffer)
        where
            Self: Sized 
    {
        match self.menu {
            MenuState::MainMenu => self.render_main_menu(area, buffer),
            MenuState::UserMenu => self.render_user_menu(area, buffer),
            MenuState::InfectedMenu => self.render_infected_menu(area, buffer),
            MenuState::StatsMenu => self.render_stats_menu(area, buffer),
        }
    }
}

pub fn menu_state_to_number(state: MenuState) -> u8 {
    match state {
        MenuState::MainMenu => 1,
        MenuState::UserMenu => 2,
        MenuState::InfectedMenu => 3,
        MenuState::StatsMenu => 4,
    }
}