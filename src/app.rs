use anyhow::{Context, Result};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::{
    stream::{FusedStream, Stream},
    task::Poll,
    StreamExt,
};
use futures_lite::StreamExt as LiteStreamExt;
use std::pin::Pin;
use std::task::Context as TaskContext;
use tokio::select;
use tor_client_lib::{control_connection::OnionServiceListener, TorServiceId};
use voynich::{
    engine::{ConnectionDirection, Engine, NetworkEvent},
    logger::{Logger, StandardLogger},
};

use crate::{
    app_context::{AppContext, ConnectionContext},
    input::{
        allow_connection_input::AllowConnectionInput, chat_input::ChatInput,
        command_input::CommandInput,
    },
    root::Root,
    term::Term,
};

#[derive(Debug)]
pub struct TermInputStream {
    reader: EventStream,
}

impl TermInputStream {
    fn new() -> Self {
        Self {
            reader: EventStream::new(),
        }
    }
}

impl Stream for TermInputStream {
    type Item = Result<Event, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.reader.poll_next(cx)
    }
}

impl FusedStream for TermInputStream {
    fn is_terminated(&self) -> bool {
        false
    }
}

pub trait InputHandler {
    async fn handle_input_event(
        &mut self,
        event: Event,
        context: &mut AppContext,
        engine: &mut Engine,
        logger: &mut StandardLogger,
    );
}

#[derive(Debug)]
pub struct App {
    term: Term,
    input_stream: TermInputStream,
    context: AppContext,
    chat_input: ChatInput,
    command_input: CommandInput,
    allow_connection_input: AllowConnectionInput,
}

impl App {
    fn new(id: TorServiceId, onion_service_address: String) -> Result<Self> {
        Ok(Self {
            term: Term::start()?,
            input_stream: TermInputStream::new(),
            context: AppContext::new(id, onion_service_address),
            chat_input: ChatInput::new(),
            command_input: CommandInput::new(),
            allow_connection_input: AllowConnectionInput::new(),
        })
    }

    pub async fn run(
        engine: &mut Engine,
        listener: &OnionServiceListener,
        logger: &mut StandardLogger,
    ) -> Result<()> {
        install_panic_hook();
        let mut app = Self::new(engine.id(), engine.onion_service_address())?;

        logger.log_info(&format!(
            "Onion service {} in service",
            engine.onion_service_address(),
        ));

        logger.log_info("NOTE: To bring up the help screen, type ctrl-h");

        while !app.context.should_quit {
            app.draw(logger)?;
            app.handle_events(engine, listener, logger).await?;
        }
        Term::stop()?;
        Ok(())
    }

    fn draw(&mut self, logger: &mut StandardLogger) -> Result<()> {
        self.term
            .draw(|frame| {
                let mut root =
                    Root::new(&self.context, logger, &self.command_input, &self.chat_input);
                if let Some((x, y)) = root.get_cursor_location(frame.size()) {
                    frame.set_cursor(x, y);
                }
                frame.render_widget(root, frame.size());
            })
            .context("terminal.draw")?;
        Ok(())
    }

    async fn handle_events(
        &mut self,
        engine: &mut Engine,
        listener: &OnionServiceListener,
        logger: &mut StandardLogger,
    ) -> Result<()> {
        select! {
            result = self.input_stream.select_next_some() => {
                match result {
                    Ok(event) => {
                        self.handle_input_event(event, engine, logger).await;
                        Ok(())
                    },
                    Err(error) => {
                        logger.log_error(&format!("Error reading input: {}", error));
                        Ok(())
                    },
                }
            }
            result = engine.get_event(logger) => {
                match result {
                    Ok(Some(NetworkEvent::NewConnection(connection))) => {
                        if *connection.direction() == ConnectionDirection::Incoming {
                            self.context.connection_context = Some(ConnectionContext::new(&connection.id()));
                        } else {
                            self.context.add_new_chat(&connection.id());
                        }
                        Ok(())
                    }
                    Ok(Some(NetworkEvent::Message(chat_message))) => {
                        if let Some(chat) = self.context.chats.get_mut(&chat_message.sender) {
                            chat.add_message(*chat_message);
                        }
                        Ok(())
                    },
                    Ok(Some(NetworkEvent::ConnectionClosed(connection))) => {
                        self.context.chat_list.remove(&connection.id());
                        self.context.chats.remove(&connection.id());
                        self.context.remove_id(&connection.id());
                        Ok(())
                    }
                    Ok(None) => Ok(()),
                    Err(error) => Err(error),
                }
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, socket_addr)) => {
                        engine.handle_incoming_connection(stream, socket_addr).await;
                    },
                    Err(error) => {
                        logger.log_error(&format!("Error in accept: {}", error));
                    }
                }
                Ok(())
            }
        }
    }

    async fn handle_input_event(
        &mut self,
        event: Event,
        engine: &mut Engine,
        logger: &mut StandardLogger,
    ) {
        if self.context.connection_context.is_some() {
            self.allow_connection_input
                .handle_input_event(event, &mut self.context, engine, logger)
                .await;
        } else if self.context.show_command_popup {
            self.command_input
                .handle_input_event(event, &mut self.context, engine, logger)
                .await;
        } else if self.context.chat_list.current_index().is_some() {
            self.chat_input
                .handle_input_event(event, &mut self.context, engine, logger)
                .await;
        } else if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: _,
            state: _,
        }) = event
        {
            match code {
                KeyCode::Char(character) => {
                    if character == 'c' && modifiers.contains(KeyModifiers::CONTROL) {
                        self.context.should_quit = true;
                    } else if character == 'k' && modifiers.contains(KeyModifiers::CONTROL) {
                        self.context.show_command_popup = !self.context.show_command_popup;
                        if self.context.show_command_popup {
                            self.context.show_welcome_popup = false;
                        }
                    } else if character == 'h' && modifiers.contains(KeyModifiers::CONTROL) {
                        self.context.show_welcome_popup = !self.context.show_welcome_popup;
                        if self.context.show_welcome_popup {
                            self.context.show_command_popup = false;
                        }
                    }
                }
                KeyCode::Esc => {
                    if self.context.show_welcome_popup {
                        self.context.show_welcome_popup = false;
                    }
                    if self.context.show_command_popup {
                        self.context.show_command_popup = false;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn install_panic_hook() {
    better_panic::install();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Term::stop();
        hook(info);
        std::process::exit(1);
    }));
}
