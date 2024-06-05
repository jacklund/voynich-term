use crate::{
    app::InputHandler,
    app_context::AppContext,
    input::{CursorMovement, Input},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use voynich::{
    chat::ChatMessage,
    engine::Engine,
    logger::{Logger, StandardLogger},
};

#[derive(Debug)]
pub struct ChatInput {
    input: Input,
}

impl ChatInput {
    pub fn new() -> Self {
        Self {
            input: Input::new(None),
        }
    }

    pub fn get_input(&self) -> String {
        self.input.get_input()
    }

    pub fn cursor_location(&self, width: usize) -> (u16, u16) {
        self.input.cursor_location(width)
    }
}

impl InputHandler for ChatInput {
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
                    if context.show_welcome_popup {
                        context.show_welcome_popup = false;
                    }
                }
                KeyCode::Enter => {
                    if let Some(input) = self.input.reset_input() {
                        match context.chat_list.current_index() {
                            Some(_) => {
                                let id = context.chat_list.current().unwrap().clone();
                                match context.chats.get_mut(&id) {
                                    Some(chat) => {
                                        if let Some(command) = input.strip_prefix('/') {
                                            match command {
                                                "quit" => {
                                                    let _ = engine.disconnect(&id, logger).await;
                                                    context.chats.remove(&id);
                                                    context.chat_list.remove(&id);
                                                }
                                                _ => logger.log_error(&format!(
                                                    "Unknown command '{}'",
                                                    &input[1..]
                                                )),
                                            }
                                        } else {
                                            let message =
                                                ChatMessage::new(&context.id, &id, input.clone());
                                            chat.add_message(message.clone());
                                            if let Err(error) =
                                                engine.send_message(message, logger).await
                                            {
                                                logger.log_error(&format!(
                                                    "Error sending chat message: {}",
                                                    error
                                                ));
                                            }
                                        }
                                    }
                                    None => {
                                        logger.log_error("No current chat");
                                    }
                                }
                            }
                            None => {
                                logger.log_error("No current chat");
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
                    if modifiers == KeyModifiers::CONTROL {
                        context.chat_list.prev_chat();
                    } else {
                        self.input.move_cursor(CursorMovement::Left);
                    }
                }
                KeyCode::Right => {
                    if modifiers == KeyModifiers::CONTROL {
                        context.chat_list.next_chat();
                    } else {
                        self.input.move_cursor(CursorMovement::Right);
                    }
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
