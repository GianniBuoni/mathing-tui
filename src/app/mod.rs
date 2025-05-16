use std::{collections::HashMap, error::Error};

use crate::prelude::*;
use crossterm::event::{KeyCode, KeyModifiers};

pub mod prelude {
    pub use super::App;
    pub(crate) use super::actions::Action;
    pub(crate) use super::views::CurrentModel;
}

mod actions;
mod models;
#[cfg(test)]
mod tests;
mod views;

#[derive(Debug, Default)]
pub struct App {
    models: HashMap<CurrentModel, Box<dyn Model>>,
    current_model: CurrentModel,
    should_exit: bool,
}

impl App {
    pub async fn run(&mut self, mut tui: Tui) -> Result<(), Box<dyn Error>> {
        while !self.should_exit {
            let event = tui.next_event().await;
            let action = self.handle_events(event);
            self.update(action);

            tui.terminal.draw(|frame| {
                self.render(frame.area(), frame.buffer_mut());
            })?;
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
                    (KeyCode::Tab, KeyModifiers::NONE) => {
                        Some(Action::SwitchPane)
                    }
                    (KeyCode::Char('j'), KeyModifiers::NONE)
                    | (KeyCode::Down, KeyModifiers::NONE) => {
                        Some(Action::TableNavigateDown)
                    }
                    (KeyCode::Char('k'), KeyModifiers::NONE)
                    | (KeyCode::Up, KeyModifiers::NONE) => {
                        Some(Action::TableNavigateUp)
                    }
                    _ => None,
                }
            }
            Some(_) => None,
            None => None,
        }
    }
}
