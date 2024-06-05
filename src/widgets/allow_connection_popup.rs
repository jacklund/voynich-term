use crate::{
    root::centered_rect,
    theme::{LIGHT_GRAY, THEME},
};
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct AllowConnectionPopup {
    onion_address: String,
    accept_selected: bool,
}

impl AllowConnectionPopup {
    pub fn new(onion_address: &str, accept_selected: bool) -> Self {
        Self {
            onion_address: onion_address.to_string(),
            accept_selected,
        }
    }
}

impl Widget for AllowConnectionPopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = "Incoming Connection Attempt";
        let intro = "  Incoming connection from:";
        let address = format!("{}.onion", self.onion_address);
        let accept_text = "<Accept>";
        let reject_text = "<Reject>";

        let message_width = std::cmp::max(intro.len(), address.len());
        let num_spaces = message_width - accept_text.len() - reject_text.len();
        let mut spacer = String::new();
        for _ in 0..num_spaces {
            spacer.push(' ');
        }

        let mut buttons = Line::default();
        let selected_style = Style::default().add_modifier(Modifier::BOLD).bg(LIGHT_GRAY);
        let unselected_style = Style::default();
        let (accept_button, reject_button) = if self.accept_selected {
            (
                Span::styled(accept_text, selected_style),
                Span::styled(reject_text, unselected_style),
            )
        } else {
            (
                Span::styled(accept_text, unselected_style),
                Span::styled(reject_text, selected_style),
            )
        };
        buttons.spans = vec![accept_button, Span::raw(spacer), reject_button];

        let message_text = vec![
            Line::styled(title, Style::default().add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            Line::raw(""),
            Line::raw(intro),
            Line::raw(""),
            Line::styled(address, Style::default().add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            Line::raw(""),
            buttons.alignment(Alignment::Center),
        ];

        let area = centered_rect(
            Constraint::Length((message_width + 6) as u16),
            Constraint::Length(message_text.len() as u16 + 2),
            area,
        );
        let message = Paragraph::new(message_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(THEME.input_panel.border),
        );
        Clear.render(area, buf); //this clears out the background
        message.render(area, buf);
    }
}
