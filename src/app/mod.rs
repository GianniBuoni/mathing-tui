use std::{collections::HashMap, error::Error, sync::mpsc::Receiver};

use crate::prelude::*;
use ratatui::DefaultTerminal;

pub mod prelude {
    pub use super::App;
    pub(crate) use super::views::CurrentModel;
}

mod default;
#[cfg(test)]
mod tests;
mod views;

pub struct App {
    models: HashMap<CurrentModel, Box<dyn Model>>,
    current_model: CurrentModel,
    should_exit: bool,
}

impl App {
    pub fn run(
        &mut self,
        mut terminal: DefaultTerminal,
        rx: Receiver<event::KeyEvent>,
    ) -> Result<(), Box<dyn Error>> {
        // first draw before event loop
        terminal.draw(|frame| {
            self.render(frame.area(), frame.buffer_mut());
        })?;

        while !self.should_exit {
            self.handle_key_events(rx.recv()?);

            terminal.draw(|frame| {
                self.render(frame.area(), frame.buffer_mut());
            })?;
        }
        Ok(())
    }

    pub fn list_models(&self) -> Vec<&dyn Model> {
        let mut models = self
            .models
            .values()
            .map(|model| model.as_ref())
            .collect::<Vec<&dyn Model>>();
        models.sort_by_key(|model| model.index());
        models
    }

    fn handle_key_events(&mut self, key_event: event::KeyEvent) {
        if key_event.kind != event::KeyEventKind::Press {
            return;
        }
        match key_event.code {
            event::KeyCode::Char('q') => {
                self.should_exit = true;
            }
            event::KeyCode::Tab => {
                self.cycle_view();
            }
            event::KeyCode::Char('j') => {
                if let Some(model) = self.models.get_mut(&self.current_model) {
                    model.next_row();
                }
            }
            event::KeyCode::Char('k') => {
                if let Some(model) = self.models.get_mut(&self.current_model) {
                    model.prev_row();
                }
            }
            _ => {}
        }
    }
}
