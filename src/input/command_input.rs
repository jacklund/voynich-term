use crate::{
    app::InputHandler,
    app_context::AppContext,
    commands::Command,
    input::{CursorMovement, Input},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::str::FromStr;
use voynich::{
    engine::Engine,
    logger::{Logger, StandardLogger},
};

#[derive(Debug)]
pub struct CommandInput {
    input: Input,
}

impl CommandInput {
    pub fn new() -> Self {
        Self {
            input: Input::new(Some(":> ")),
        }
    }

    pub fn get_input(&self) -> String {
        self.input.get_input()
    }

    pub fn cursor_location(&self, width: usize) -> (u16, u16) {
        self.input.cursor_location(width)
    }

    pub async fn handle_command(
        &mut self,
        context: &mut AppContext,
        logger: &mut StandardLogger,
        command: Command,
        engine: &mut Engine,
    ) {
        match command {
            Command::Connect { address } => {
                if let Err(error) = engine.connect(&address).await {
                    logger.log_error(&format!("Connect error: {}", error));
                }
            }
            Command::Quit => {
                context.should_quit = true;
            }
        }
    }
}

impl InputHandler for CommandInput {
    async fn handle_input_event(
        &mut self,
        event: Event,
        context: &mut AppContext,
        engine: &mut Engine,
        logger: &mut StandardLogger,
    ) {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                KeyCode::Char(character) => {
                    if character == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                        context.should_quit = true;
                    } else if character == 'k' && modifiers.contains(KeyModifiers::CONTROL) {
                        context.toggle_command_popup();
                        if context.show_command_popup {
                            context.show_welcome_popup = false;
                        }
                    } else if character == 'u' && modifiers.contains(KeyModifiers::CONTROL) {
                        self.input.clear_input_to_cursor();
                    } else if character == 'h' && modifiers.contains(KeyModifiers::CONTROL) {
                        context.toggle_welcome_popup();
                        if context.show_welcome_popup {
                            context.show_command_popup = false;
                        }
                    } else {
                        self.input.write(character);
                    }
                }
                KeyCode::Esc => {
                    context.show_command_popup = false;
                }
                KeyCode::Enter => {
                    if let Some(input) = self.input.reset_input() {
                        context.toggle_command_popup();
                        match Command::from_str(&input) {
                            Ok(command) => {
                                self.handle_command(context, logger, command, engine).await;
                            }
                            Err(error) => {
                                logger.log_error(&format!("Error parsing command: {}", error));
                            }
                        }
                    }
                }
                KeyCode::Delete => {
                    self.input.remove();
                }
                KeyCode::Backspace => {
                    self.input.remove_previous();
                }
                KeyCode::Left => {
                    self.input.move_cursor(CursorMovement::Left);
                }
                KeyCode::Right => {
                    self.input.move_cursor(CursorMovement::Right);
                }
                KeyCode::Home => {
                    self.input.move_cursor(CursorMovement::Start);
                }
                KeyCode::End => {
                    self.input.move_cursor(CursorMovement::End);
                }
                _ => {}
            }
        }
    }
}
