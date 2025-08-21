use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, ListState, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::{domains::infected::Infected, interface::app::*, repos::database::{InfectedDatabase, InfectedRepo}};

impl App {
    pub fn render_infected_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
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
                let infected = Infected::new("Windows PC", "127.0.0.1");

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
}