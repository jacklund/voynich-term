use crate::theme::THEME;
use ratatui::{prelude::*, widgets::block::*, widgets::*};
use voynich::chat::ChatList;

pub struct ChatTabs<'a> {
    lines: Vec<Line<'a>>,
    current_index: usize,
}

impl<'a> ChatTabs<'a> {
    pub fn new(chat_list: &ChatList) -> Self {
        let lines = chat_list
            .names()
            .iter()
            .map(|s| Line::from(s.as_str().to_string()))
            .collect();
        Self {
            lines,
            current_index: chat_list.current_index().unwrap(),
        }
    }
}

impl<'a> Widget for ChatTabs<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Tabs::new(self.lines)
            .block(Block::default().title("Chats").borders(Borders::ALL))
            .style(THEME.chat_tabs.style)
            .highlight_style(THEME.chat_tabs.highlight_style)
            .select(self.current_index)
            .render(area, buf);
    }
}
