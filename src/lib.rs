use crate::prelude::*;
use ratatui::DefaultTerminal;

pub mod prelude {
    pub use super::App;
    pub use ratatui::{crossterm::event, prelude::*, widgets::*};
    pub use std::io;
}

mod ui;

#[derive(Default)]
pub struct App {
    should_exit: bool,
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| {
            self.render(frame.area(), frame.buffer_mut());
        })?;

        while !self.should_exit {
            match event::read()? {
                event::Event::Key(key_event) => {
                    self.handle_key_events(key_event);
                }
                _ => {}
            }

            terminal.draw(|frame| {
                self.render(frame.area(), frame.buffer_mut());
            })?;
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: event::KeyEvent) {
        if key_event.kind != event::KeyEventKind::Press {
            return;
        }
        match key_event.code {
            event::KeyCode::Char('q') => {
                self.should_exit = true;
            }
            _ => {}
        }
    }
}
