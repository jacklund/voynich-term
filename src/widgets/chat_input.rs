use crate::{input::chat_input::ChatInput, root::split_each, theme::THEME};
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct ChatInputWidget<'a> {
    input: &'a ChatInput,
    length: u16,
}

impl<'a> ChatInputWidget<'a> {
    pub fn new(chat_input: &'a ChatInput) -> Self {
        Self {
            input: chat_input,
            length: 0,
        }
    }

    pub fn cursor_location(&mut self, inner_width: usize) -> (u16, u16) {
        let (x, y) = self.input.cursor_location(inner_width);
        self.length = y + 1;
        (x, y)
    }

    pub fn get_length(&self) -> u16 {
        self.length
    }
}

impl<'a> Widget for ChatInputWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_width = (area.width - 2) as usize;

        let input = split_each(self.input.get_input(), inner_width)
            .into_iter()
            .map(|line| Line::from(vec![Span::raw(line)]))
            .collect::<Vec<_>>();

        Paragraph::new(input)
            .block(Block::default().borders(Borders::NONE))
            .style(THEME.chat_input)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}
