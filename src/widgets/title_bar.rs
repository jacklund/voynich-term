use crate::theme::THEME;
use clap::{crate_name, crate_version};
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct TitleBar {
    onion_service_address: String,
}

impl TitleBar {
    pub fn new(onion_service_address: &str) -> Self {
        Self {
            onion_service_address: onion_service_address.to_string(),
        }
    }
}

impl Widget for TitleBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from(vec![Span::styled(
            format!(
                "{} {}  Onion address: {}",
                crate_name!(),
                crate_version!(),
                self.onion_service_address
            ),
            Style::new().add_modifier(Modifier::BOLD),
        )]))
        .block(Block::default().borders(Borders::NONE))
        .style(THEME.title_bar)
        .alignment(Alignment::Left)
        .render(area, buf);
    }
}
