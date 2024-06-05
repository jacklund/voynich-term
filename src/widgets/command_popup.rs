use crate::{
    input::command_input::CommandInput,
    root::{centered_rect, split_each},
    theme::THEME,
};
use ratatui::{prelude::*, widgets::block::*, widgets::*};

pub struct CommandPopup<'a> {
    command_input: &'a CommandInput,
    render_area: Option<Rect>,
}

impl<'a> CommandPopup<'a> {
    pub fn new(command_input: &'a CommandInput) -> Self {
        Self {
            command_input,
            render_area: None,
        }
    }

    pub fn get_cursor_location(&mut self, area: Rect) -> Option<(u16, u16)> {
        // Calculate the width of the render area
        let popup_area = centered_rect(Constraint::Percentage(70), Constraint::Length(3), area);
        let inner_width = (popup_area.width - 2) as usize;

        // Calculate where the cursor is relative to the popup
        let input_cursor = self.command_input.cursor_location(inner_width);

        // Calculate the rendered popup area
        let popup_area = centered_rect(
            Constraint::Percentage(70),
            Constraint::Length(input_cursor.1 + 3),
            area,
        );
        // Save it
        self.render_area = Some(popup_area);

        Some((
            popup_area.x + input_cursor.0 + 1,
            popup_area.y + input_cursor.1 + 1,
        ))
    }
}

impl<'a> Widget for CommandPopup<'a> {
    fn render(self, _area: Rect, buf: &mut Buffer) {
        // Get the input string
        let input_string = self.command_input.get_input();

        // Calculate the inner width
        let inner_width = (self.render_area.unwrap().width - 2) as usize;

        // Split the string according to width
        let split_input = split_each(input_string, inner_width)
            .into_iter()
            .map(|line| Line::from(vec![Span::raw(line)]))
            .collect::<Vec<_>>();

        // Generate the input panel
        let input_panel = Paragraph::new(split_input)
            .block(
                Block::default()
                    .title(Line::styled("Command Input", THEME.input_panel.title))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .border_style(THEME.input_panel.border),
            )
            .style(THEME.input_panel.style)
            .alignment(Alignment::Left);

        // Clear and render
        Clear.render(self.render_area.unwrap(), buf); //this clears out the background
        input_panel.render(self.render_area.unwrap(), buf);
    }
}
