use ratatui::buffer::Buffer;

use crate::interface::app::*;

impl App {
    pub fn render_main_menu(&self, area: ratatui::prelude::Rect, buffer: &mut Buffer) {
        self.default_menu_instruction(" Main ", area, buffer)
    }
}