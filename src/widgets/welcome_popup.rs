use crate::{root::centered_rect, theme::THEME};
use clap::{crate_name, crate_version};
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct WelcomePopup {
    onion_service_address: String,
}

impl WelcomePopup {
    pub fn new(onion_service_address: &str) -> Self {
        Self {
            onion_service_address: onion_service_address.to_string(),
        }
    }
}

impl Widget for WelcomePopup {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!("Welcome to {} version {}", crate_name!(), crate_version!());
        let address = format!(
            "Your onion service address is: {}",
            self.onion_service_address
        );
        let greeting_text = vec![
            Line::styled(title, Style::default().add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center),
            Line::raw(""),
            Line::raw(address),
            Line::raw(""),
            Line::styled(
                "Getting Started",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Line::raw(""),
            Line::raw("To connect to someone, press ctrl-k to bring up a command window, and type 'connect <onion-address>'"),
            Line::raw("Once connected, type your messages in the input box at the bottom"),
            Line::raw("To quit a chat, type '/quit' in the chat input box"),
            Line::raw("To run a command (listed below) type ctrl-k and type the command"),
            Line::raw("Type ctrl-c anywhere, or 'quit' in the command window, to exit"),
            Line::raw("Type ctrl-h to show/hide this window again"),
            Line::raw("Type ctrl-k to show/hide the command window"),
            Line::raw(""),
            Line::styled(
                "Commands",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Line::raw(""),
            Line::raw("connect <address>  - to connect to another chat user"),
            Line::raw("quit               - to exit the application"),
            Line::raw(""),
        ];
        let greeting_width = greeting_text
            .iter()
            .map(|l| l.width())
            .max_by(|x, y| x.cmp(y))
            .unwrap();

        let area = centered_rect(
            Constraint::Length((greeting_width + 2) as u16),
            Constraint::Length(greeting_text.len() as u16 + 2),
            area,
        );
        let greeting = Paragraph::new(greeting_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(THEME.input_panel.border),
        );
        Clear.render(area, buf); //this clears out the background
        greeting.render(area, buf);
    }
}
