mod widgets;

use crate::processor::Processor;

use ratatui::prelude::*;
use crossterm::{terminal, event::{self, *}, ExecutableCommand};

use std::time::Duration;
use std::io;

macro_rules! lock {
    ($mutex:expr) => {
        $mutex.lock().map_err(|_| Into::<Box<dyn std::error::Error>>::into("failed to lock"))
    }
}

pub enum Step {
    Once,
    Forever,
    Never,
}

impl Step {
    pub fn should_step(&mut self) -> bool {
        match *self {
            Step::Once => { *self = Step::Never; true },
            Step::Forever => true,
            Step::Never => false,
        }
    }

    pub fn toggle(&mut self) {
        match *self {
            Step::Once | Step::Forever => *self = Step::Never,
            Step::Never => *self = Step::Forever,
        }
    }
}

pub struct Tui {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    processor: Processor,
    step: Step,
    should_close: bool,
}

impl Tui {
    pub fn new() -> Result<Tui, Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        io::stdout().execute(terminal::EnterAlternateScreen)?;

        Ok(Tui {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout()))?,
            processor: Processor::new(),
            step: Step::Never,
            should_close: false,
        })
    }

    fn handle_keypress(&mut self, keycode: KeyCode) -> Result<(), Box<dyn std::error::Error>> {
        match keycode {
            KeyCode::Char(' ') => self.step.toggle(),
            KeyCode::Enter => self.step = Step::Once,
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
        self.processor.flash(rom)
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.should_close {
            self.poll_event()?;

            let processor = self.processor.clone();

            self.terminal.draw(move |frame| {
                widgets::draw(frame, processor);
            })?;

            if self.step.should_step() {
                self.processor.step();
            }
        }

        io::stdout().execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}


