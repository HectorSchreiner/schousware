use std::{default, io, net::{IpAddr, Ipv4Addr}, process::id, str::FromStr, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, ListState, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::domains::{c4server::{self, *}, infected::*};
use crate::repos::database::*;

pub struct App {
    pub menu: AppMenuState,
    pub exit: ExitState,
    pub c2server: C4Server,
    pub infected_database: InfectedDatabase,
}

#[derive(Debug, Default)]
pub enum ExitState {
    #[default]
    Running,
    Exit,
}

#[derive(Debug, Default)]
pub enum AppMenuState {
    #[default]
    MainMenu,
    UserMenu,
    InfectedMenu(InfectedMenuState),
    StatsMenu,
}
#[derive(Debug, Default)]
pub enum InfectedMenuState {
    #[default]
    ShowInfected,
    AddMachine,
}

pub struct InfectedList {
    pub items: Vec<InfectedItem>,
    pub state: ListState
}

pub struct InfectedItem {
    pub infected: Infected,
    pub connetion_status: InfectedConnectionStatus,
}

impl InfectedList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn from_database(db: &InfectedDatabase) -> Self {
        let mut infected_list = Vec::new();
        db.get_all_infected().unwrap().iter().for_each(|i| {
            infected_list.push(InfectedItem { infected: i.clone(), connetion_status: InfectedConnectionStatus::Disconnected });
        });

        InfectedList { items: infected_list, state: ListState::default() }
    }

    pub fn with_items(items: Vec<InfectedItem>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        Self { items, state }
    }
}

impl App {
    pub fn default() -> Self {
        // create and init a new fileserver
        //let source = std::env::current_dir().unwrap();

        let infected_database =  InfectedDatabase::new().expect("Could not create database");

        Self { 
            menu: AppMenuState::default(), 
            exit: ExitState::default(), 
            c2server: C4Server::default(), 
            infected_database
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        //self.file_server.serve().expect("Could not initialize the fileserver");
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        //self.file_server.close();
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {},
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.menu {
            AppMenuState::InfectedMenu(_) => self.handle_infected_key_event(key_event),
            _ => {}
        }
        match key_event.code {
            KeyCode::Esc => self.exit = ExitState::Exit,
            KeyCode::Char('1') => self.menu = AppMenuState::MainMenu,
            KeyCode::Char('2') => self.menu = AppMenuState::UserMenu,
            KeyCode::Char('3') => self.menu = AppMenuState::InfectedMenu(InfectedMenuState::default()),
            KeyCode::Char('4') => self.menu = AppMenuState::StatsMenu,
            _ => {}
        }
    }

    pub fn handle_infected_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('a')  => self.menu = AppMenuState::InfectedMenu(InfectedMenuState::AddMachine),
            KeyCode::Char('s')  => self.menu = AppMenuState::InfectedMenu(InfectedMenuState::ShowInfected),
            _ => {}
        }
    }
    
    pub fn is_running(&self) -> bool {
        match self.exit {
            ExitState::Exit => false,
            ExitState::Running => true,
        }
    }

    pub fn menu_selection(&self) -> Line<'static> {
        Line::from(" <1>Main <2>Users <3>Infected <4>Stats")
    } 

    pub fn menu_instructions(&self) -> Line<'static> {
        Line::from(vec![
            " Exit ".into(),
            "<ESC>".into(),
            " Change Menu ".into(),
            "<NUM> ".into()
            ])
    }

    pub fn default_menu_instruction(&self, title: &'static str, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        let title = Line::from(title);

        return Block::bordered()
        .title(title.centered())
        .title_top(self.menu_instructions().left_aligned())
        .title_bottom(self.menu_selection().left_aligned())
        .border_set(border::ROUNDED)
        .render(area, buffer)
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buffer: &mut Buffer)
        where
            Self: Sized 
    {
        match self.menu {
            AppMenuState::MainMenu => self.render_main_menu(area, buffer),
            AppMenuState::UserMenu => self.render_user_menu(area, buffer),
            AppMenuState::InfectedMenu(_) => self.render_infected_menu(area, buffer),
            AppMenuState::StatsMenu => self.render_stats_menu(area, buffer),
        }
    }
}

pub fn menu_state_to_number(state: AppMenuState) -> u8 {
    match state {
        AppMenuState::MainMenu => 1,
        AppMenuState::UserMenu => 2,
        AppMenuState::InfectedMenu(_) => 3,
        AppMenuState::StatsMenu => 4,
    }
}