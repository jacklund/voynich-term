pub mod allow_connection_input;
pub mod chat_input;
pub mod command_input;

pub enum CursorMovement {
    Left,
    Right,
    Start,
    End,
}

// pub enum ScrollMovement {
//     Up,
//     Down,
//     Start,
// }

#[derive(Clone, Debug, Default)]
pub struct Input {
    buffer: Vec<char>,
    cursor: usize,
    prompt_size: usize,
}

impl Input {
    pub fn new(prompt: Option<&str>) -> Self {
        let mut buffer = Vec::new();
        let prompt_size = match prompt {
            Some(prompt) => {
                buffer.extend_from_slice(prompt.chars().collect::<Vec<char>>().as_slice());
                prompt.len()
            }
            None => 0,
        };
        Self {
            buffer,
            cursor: prompt_size,
            prompt_size,
        }
    }

    pub fn get_input(&self) -> String {
        self.buffer[..].iter().collect::<String>()
    }

    pub fn write(&mut self, character: char) {
        self.buffer.insert(self.cursor, character);
        self.cursor += 1;
    }

    pub fn remove(&mut self) {
        if self.cursor < self.buffer.len() {
            self.buffer.remove(self.cursor);
        }
    }

    pub fn remove_previous(&mut self) {
        if self.cursor > self.prompt_size {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
        }
    }

    pub fn move_cursor(&mut self, movement: CursorMovement) {
        match movement {
            CursorMovement::Left => {
                if self.cursor > self.prompt_size {
                    self.cursor -= 1;
                }
            }
            CursorMovement::Right => {
                if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
            }
            CursorMovement::Start => {
                self.cursor = self.prompt_size;
            }
            CursorMovement::End => {
                self.cursor = self.buffer.len();
            }
        }
    }

    pub fn clear_input_to_cursor(&mut self) {
        if !self.buffer.is_empty() {
            self.buffer.drain(self.prompt_size..self.cursor);
            self.cursor = self.prompt_size;
        }
    }

    pub fn reset_input(&mut self) -> Option<String> {
        if !self.buffer.is_empty() {
            self.cursor = self.prompt_size;
            return Some(self.buffer.drain(self.prompt_size..).collect());
        }
        None
    }

    pub fn cursor_location(&self, width: usize) -> (u16, u16) {
        let mut position = (0, 0);

        for current_char in self.buffer.iter().take(self.cursor) {
            let char_width = unicode_width::UnicodeWidthChar::width(*current_char).unwrap_or(0);

            position.0 += char_width;

            match position.0.cmp(&width) {
                std::cmp::Ordering::Equal => {
                    position.0 = 0;
                    position.1 += 1;
                }
                std::cmp::Ordering::Greater => {
                    // Handle a char with width > 1 at the end of the row
                    // width - (char_width - 1) accounts for the empty column(s) left behind
                    position.0 -= width - (char_width - 1);
                    position.1 += 1;
                }
                _ => (),
            }
        }

        (position.0 as u16, position.1 as u16)
    }
}
