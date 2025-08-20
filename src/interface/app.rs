use std::{default, io, net::{IpAddr, Ipv4Addr}, process::id, str::FromStr, vec};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::{domains::{self, infected::{self, HostName, Infected, InfectedId, InfectedIpAddr}}, repos::database::{self, InfectedDatabase, InfectedRepo}};

#[derive(Debug, Default)]
pub struct App {
    menu: AppMenuState,
    exit: ExitState,
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

impl App {
    pub fn init() -> Self {
        Self { menu: AppMenuState::default(), exit: ExitState::default() }
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

    fn handle_infected_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('a')  => self.menu = AppMenuState::InfectedMenu(InfectedMenuState::AddMachine),
            KeyCode::Char('s')  => self.menu = AppMenuState::InfectedMenu(InfectedMenuState::ShowInfected),
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
        let title = Line::from(" Infected ");
        let infected_commands = Line::from(vec![
            " S ".into(),
            "<Show> ".into(),
            " A ".into(),
            "<Add> ".into(),
            ]);

        Block::bordered()
        .title(title.centered())
        .title_top(self.menu_instructions().left_aligned())
        .title_bottom(self.menu_selection().left_aligned())
        .title_bottom(infected_commands.right_aligned())
        .border_set(border::ROUNDED)
        .render(area, buffer);

        let db = InfectedDatabase::new().unwrap();

        let inner_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: area.height - 2,
        };

        match self.menu {
            AppMenuState::InfectedMenu(InfectedMenuState::AddMachine) => {
                let mut info = Text::default();
                let infected = Infected { id: InfectedId::new(), hostname: HostName::new("hostname"), ip: InfectedIpAddr::from_str("127.0.0.1").unwrap() };
                match db.add_infected(&infected) {
                    Ok(_) => {
                        info.push_line(format!("Sucessfully added {:?} to database", &infected));
                    },
                    Err(err) => {
                        info.push_line(format!("Could not add {:?} to database. Error {:?}", &infected, err));
                    }
                }

                let paragraph = Paragraph::new(info).wrap(Wrap { trim: true });
                paragraph.render(inner_area, buffer);
            },
            AppMenuState::InfectedMenu(InfectedMenuState::ShowInfected) => {
                let mut machine_list_text = Text::default();
            
                if let Ok(infected_machines) = db.get_all_infected() {
                    for (i, infected) in infected_machines.iter().enumerate() {
                        machine_list_text.push_line(format!("[{:?}] Hostname: {} Ip: {} Uuid: {}", i, infected.hostname(), infected.ip(), infected.id()));
                    }
                } else {
                    machine_list_text.push_line("No Machines Found");
                }
            
                let paragraph = Paragraph::new(machine_list_text).wrap(Wrap { trim: true });
                paragraph.render(inner_area, buffer);
            },
            _ => {}
        }


    }

    fn render_stats_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Stats ", area, buffer);
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