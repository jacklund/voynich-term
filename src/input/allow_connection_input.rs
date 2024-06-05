use crate::{app::InputHandler, app_context::AppContext};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use voynich::{engine::Engine, logger::StandardLogger};

#[derive(Debug)]
pub struct AllowConnectionInput {}

impl AllowConnectionInput {
    pub fn new() -> Self {
        Self {}
    }
}

impl InputHandler for AllowConnectionInput {
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
            let accept_selected = context.connection_context.as_ref().unwrap().accept_selected;
            let connection_address = context
                .connection_context
                .as_ref()
                .unwrap()
                .connection_address
                .clone();
            match code {
                KeyCode::Char(character) => {
                    if character == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                        context.should_quit = true;
                    }
                }
                KeyCode::Esc => {
                    context.connection_context.as_mut().unwrap().accept_selected = false;
                }
                KeyCode::Enter => {
                    if accept_selected {
                        let _ = engine
                            .send_connection_authorized_message(&connection_address, logger)
                            .await;
                        context.add_new_chat(&connection_address);
                    } else {
                        engine
                            .disconnect(&connection_address, logger)
                            .await
                            .unwrap();
                    }
                    context.connection_context = None;
                }
                KeyCode::Left | KeyCode::Right | KeyCode::Tab => {
                    context.connection_context.as_mut().unwrap().accept_selected = !accept_selected;
                }
                _ => {}
            }
        }
    }
}
