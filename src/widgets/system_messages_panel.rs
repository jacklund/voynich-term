use crate::theme::{Theme, THEME};
use ratatui::{prelude::*, widgets::block::*, widgets::*};
use voynich::logger::StandardLogger;

pub struct SystemMessagesPanel<'a> {
    messages: Vec<Line<'a>>,
}

impl<'a> SystemMessagesPanel<'a> {
    pub fn new(logger: &StandardLogger) -> Self {
        let messages = logger
            .iter()
            .map(|message| {
                let date = message.date.format("%H:%M:%S ").to_string();
                let system_message_style = Theme::get_system_message_style(message);
                let ui_message = vec![
                    Span::styled(date, system_message_style.date),
                    Span::styled(message.message.clone(), system_message_style.message),
                ];
                Line::from(ui_message)
            })
            .collect::<Vec<_>>();

        Self { messages }
    }
}

impl<'a> Widget for SystemMessagesPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_height = area.height - 2;
        let scroll = if self.messages.len() as u16 > inner_height {
            self.messages.len() as u16 - inner_height
        } else {
            0
        };
        Paragraph::new(self.messages)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                "System Messages",
                Style::default().add_modifier(Modifier::BOLD),
            )))
            .style(THEME.system_messages_panel)
            .alignment(Alignment::Left)
            .scroll((scroll, 0))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
