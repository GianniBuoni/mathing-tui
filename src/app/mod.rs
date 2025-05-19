use std::error::Error;

use crate::prelude::*;
use crossterm::event::{KeyCode, KeyModifiers};

pub mod prelude {
    pub use super::App;
    pub(crate) use super::actions::Action;
}

mod actions;
#[cfg(test)]
mod tests;

#[derive(Debug, Default)]
pub struct App {
    component: Home,
    should_exit: bool,
}

impl App {
    pub async fn run(&mut self, mut tui: Tui) -> Result<(), Box<dyn Error>> {
        while !self.should_exit {
            let event = tui.next_event().await;
            let action = self.handle_events(event);
            self.update(action);

            tui.terminal
                .draw(|frame| self.component.draw(frame, frame.area()))?;
        }
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Quit) => Some(Action::Quit),
            Some(Event::Key(key_event)) => {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        Some(Action::Quit)
                    }
                    _ => self.component.handle_events(event),
                }
            }
            Some(_) => None,
            None => None,
        }
    }

    pub fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::Quit) => {
                self.should_exit = true;
            }
            Some(_) => {}
            None => {}
        }
    }
}
