mod widgets;

use crate::processor::Processor;
use crate::loader::{Hex, Kind};

use ratatui::prelude::*;
use crossterm::{terminal, event::{self, *}, ExecutableCommand};

use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::io;

macro_rules! lock {
    ($mutex:expr) => {
        $mutex.lock().map_err(|_| Into::<Box<dyn std::error::Error>>::into("failed to lock"))
    }
}


#[derive(Clone)]
pub struct Command {
    inner: Arc<Mutex<String>>,
    cursor: i32,
}

impl Command {
    pub fn new() -> Command {
        Command {
            inner: Arc::new(Mutex::new(String::new())),
            cursor: 0,
        }
    }

    pub fn move_cursor(&mut self, value: i32) {
        self.cursor += value.min(-self.cursor);
    }

    pub fn insert(&mut self, character: char) -> Result<(), Box<dyn std::error::Error>> {
        lock!(self.inner)?.insert(self.cursor as usize, character);

        self.move_cursor(1);

        Ok(())
    }

    pub fn remove(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        lock!(self.inner)?.remove(self.cursor as usize);

        self.move_cursor(-1);

        Ok(())
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    processor: Processor,
    command: Command,
    should_close: bool,
}

impl Tui {
    pub fn new() -> Result<Tui, Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        io::stdout().execute(terminal::EnterAlternateScreen)?;

        Ok(Tui {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
            processor: Processor::new(),
            command: Command::new(),
            should_close: false,
        })
    }

    // NOTE: the reason why we crash when pressing backspace is because memory is empty in this
    // example

    fn handle_keypress(&mut self, keycode: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
        match keycode {
            KeyCode::Char(character) => self.command.insert(character)?,
            KeyCode::Backspace => self.command.remove()?,
            KeyCode::Enter => self.processor.step(),
            KeyCode::Esc => self.should_close = true,
            _ => {},
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::Key(event) => {
                match event.kind {
                    KeyEventKind::Press => {
                        self.handle_keypress(event.code)?;
                    },
                    _ => {},
                }
            },
            _ => {},
        }

        Ok(())
    }

    fn poll_event(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(50))? {
            self.handle_event(event::read()?)?;
        }

        Ok(())
    }

    pub fn flash(&mut self, rom: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let mut hex = Hex::new(rom)?;

        loop {
            let record = hex.next()?;

            match record.kind {
                Kind::Data => {
                    self.processor.flash(record.addr, &record.data);
                },
                Kind::ExtendSegmentAddress => {},
                Kind::StartSegmentAddress => {
                },
                Kind::ExtendLinearAddress => {},
                Kind::StartLinearAddress => {},
                Kind::Eof => break,
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.should_close {
            self.poll_event()?;

            let command = self.command.clone();
            let processor = self.processor.clone();

            self.terminal.draw(move |frame| {
                widgets::draw(frame, command, processor);
            })?;
        }

        io::stdout().execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}


