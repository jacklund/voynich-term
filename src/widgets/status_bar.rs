use crate::theme::THEME;
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Input")
            .block(Block::default().borders(Borders::NONE))
            .style(THEME.status_bar)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
