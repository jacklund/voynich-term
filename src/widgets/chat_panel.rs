use crate::{app_context::AppContext, theme::THEME};
use ratatui::{prelude::*, widgets::block::*, widgets::*};
use tor_client_lib::TorServiceId;

pub struct ChatPanel<'a> {
    messages: Vec<Line<'a>>,
    id: TorServiceId,
}

impl<'a> ChatPanel<'a> {
    pub fn new(id: &TorServiceId, context: &'a AppContext) -> Self {
        let chat = context.chats.get(id).unwrap();
        let messages = chat
            .iter()
            .map(|message| {
                let date = message.date.format("%H:%M:%S ").to_string();
                let color = match message.sender.clone() {
                    sender_id if sender_id == *id => *context.get_color(id).unwrap(),
                    _ => Color::Blue,
                };
                let ui_message = vec![
                    Span::styled(date, THEME.chat_message.date),
                    Span::styled(message.sender.as_str(), Style::new().fg(color)),
                    Span::styled(": ", Style::new().fg(color)),
                    Span::raw(message.message.clone()),
                ];
                Line::from(ui_message)
            })
            .collect::<Vec<_>>();
        Self {
            messages,
            id: id.clone(),
        }
    }
}

impl<'a> Widget for ChatPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_height = area.height - 2;
        let scroll = if self.messages.len() as u16 > inner_height {
            self.messages.len() as u16 - inner_height
        } else {
            0
        };
        Paragraph::new(self.messages)
            .block(Block::default().borders(Borders::ALL).title(Span::styled(
                self.id.to_string(),
                Style::default().add_modifier(Modifier::BOLD),
            )))
            .style(THEME.chat_panel)
            .alignment(Alignment::Left)
            .scroll((scroll, 0))
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
