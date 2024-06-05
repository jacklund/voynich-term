use std::{
    io::{self, stdout, Error, Stdout},
    ops::{Deref, DerefMut},
};

use anyhow::{Context, Result};
use crossterm::{
    event::{Event, EventStream},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::{stream::Stream, task::Poll};
use futures_lite::StreamExt as LiteStreamExt;
use ratatui::prelude::*;
use std::pin::Pin;
use std::task::Context as TaskContext;

/// A wrapper around the terminal that handles setting up and tearing down the terminal
/// and provides a helper method to read events from the terminal.
#[derive(Debug)]
pub struct Term {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    reader: EventStream,
}

impl Term {
    pub fn start() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        enable_raw_mode().context("enable raw mode")?;
        stdout()
            .execute(EnterAlternateScreen)
            .context("enter alternate screen")?;
        Ok(Self {
            terminal,
            reader: EventStream::new(),
        })
    }

    pub fn stop() -> Result<()> {
        disable_raw_mode().context("disable raw mode")?;
        stdout()
            .execute(LeaveAlternateScreen)
            .context("leave alternate screen")?;
        Ok(())
    }
}

impl Stream for Term {
    type Item = Result<Event, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        self.reader.poll_next(cx)
    }
}

impl Deref for Term {
    type Target = Terminal<CrosstermBackend<Stdout>>;
    fn deref(&self) -> &Self::Target {
        &self.terminal
    }
}

impl DerefMut for Term {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.terminal
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let _ = Term::stop();
    }
}
