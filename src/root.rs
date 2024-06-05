use std::rc::Rc;

use ratatui::prelude::*;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use voynich::logger::StandardLogger;

use crate::{
    app_context::AppContext,
    input::{chat_input::ChatInput, command_input::CommandInput},
    widgets::{
        allow_connection_popup::AllowConnectionPopup, chat_input::ChatInputWidget,
        chat_panel::ChatPanel, chat_tabs::ChatTabs, command_popup::CommandPopup,
        status_bar::StatusBar, system_messages_panel::SystemMessagesPanel, title_bar::TitleBar,
        welcome_popup::WelcomePopup,
    },
};

pub struct Root<'a> {
    context: &'a AppContext,
    logger: &'a mut StandardLogger,
    command_popup: Option<CommandPopup<'a>>,
    chat_input: ChatInputWidget<'a>,
}

impl<'a> Root<'a> {
    pub fn new(
        context: &'a AppContext,
        logger: &'a mut StandardLogger,
        command_input: &'a CommandInput,
        chat_input: &'a ChatInput,
    ) -> Self {
        let command_popup = if context.show_command_popup {
            Some(CommandPopup::new(command_input))
        } else {
            None
        };

        Root {
            context,
            logger,
            command_popup,
            chat_input: ChatInputWidget::new(chat_input),
        }
    }
}

impl Widget for Root<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        match self.context.chat_list.current() {
            Some(id) => {
                let chunks = self.get_layout(area);

                TitleBar::new(&self.context.onion_service_address).render(chunks[0], buf);
                SystemMessagesPanel::new(self.logger).render(chunks[1], buf);
                ChatTabs::new(&self.context.chat_list).render(chunks[2], buf);
                ChatPanel::new(id, self.context).render(chunks[3], buf);
                StatusBar::new().render(chunks[4], buf);
                self.chat_input.render(chunks[5], buf);
            }
            None => {
                let chunks = self.get_layout(area);

                TitleBar::new(&self.context.onion_service_address).render(chunks[0], buf);
                SystemMessagesPanel::new(self.logger).render(chunks[1], buf);
            }
        }
        if self.context.show_command_popup {
            self.command_popup.unwrap().render(area, buf);
        }
        if self.context.show_welcome_popup {
            WelcomePopup::new(&self.context.onion_service_address).render(area, buf);
        }
        if self.context.connection_context.is_some() {
            let connection_context = self.context.connection_context.as_ref().unwrap();
            AllowConnectionPopup::new(
                &connection_context.connection_address.to_string(),
                connection_context.accept_selected,
            )
            .render(area, buf);
        }
    }
}

// split messages to fit the width of the ui panel
pub fn split_each(input: String, width: usize) -> Vec<String> {
    let mut splitted = Vec::with_capacity(input.width() / width);
    let mut row = String::new();

    let mut index = 0;

    for current_char in input.chars() {
        if (index != 0 && index == width) || index + current_char.width().unwrap_or(0) > width {
            splitted.push(std::mem::take(&mut row));
            index = 0;
        }

        row.push(current_char);
        index += current_char.width().unwrap_or(0);
    }
    // leftover
    if !row.is_empty() {
        splitted.push(std::mem::take(&mut row));
    }
    splitted
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(constraint_x: Constraint, constraint_y: Constraint, r: Rect) -> Rect {
    let vertical_constraints = match constraint_y {
        Constraint::Percentage(percent_y) => [
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ],
        Constraint::Length(length_y) => [
            Constraint::Min((r.height - length_y) / 2),
            Constraint::Min(length_y),
            Constraint::Min(((r.height - length_y) / 2) - 2),
        ],
        _ => panic!("Expected Length or Percentage, got {}", constraint_y),
    };
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_constraints)
        .split(r);

    let horizontal_constraints = match constraint_x {
        Constraint::Percentage(percent_x) => [
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ],
        Constraint::Length(length_x) => [
            Constraint::Min((r.width - length_x) / 2),
            Constraint::Percentage(length_x),
            Constraint::Min((r.width - length_x) / 2),
        ],
        _ => panic!("Expected Length or Percentage, got {}", constraint_y),
    };
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(horizontal_constraints)
        .split(popup_layout[1])[1]
}

impl Root<'_> {
    pub fn get_cursor_location(&mut self, area: Rect) -> Option<(u16, u16)> {
        if self.context.show_command_popup {
            self.command_popup
                .as_mut()
                .unwrap()
                .get_cursor_location(area)
        } else {
            let chunks = self.get_layout(area);
            if chunks.len() < 6 {
                return None;
            }
            let inner_width = (area.width - 2) as usize;
            let input_cursor = self.chat_input.cursor_location(inner_width);
            Some((area.x + input_cursor.0, chunks[5].y + input_cursor.1 + 1))
        }
    }

    fn get_layout(&mut self, area: Rect) -> Rc<[Rect]> {
        match self.context.chat_list.current() {
            Some(_) => Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Percentage(20),
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(1),
                        Constraint::Length(self.chat_input.get_length()),
                    ]
                    .as_ref(),
                )
                .split(area),
            None => Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                .split(area),
        }
    }
}
