use crate::prelude::*;
use ratatui::DefaultTerminal;

pub mod prelude {
    pub use super::App;
    pub(crate) use crate::items::prelude::*;
    pub(crate) use crate::model::prelude::*;
    pub(crate) use crate::receipt::prelude::*;
    pub(crate) use crate::views::prelude::*;
    pub use ratatui::{crossterm::event, prelude::*, widgets::*};
    pub use std::io;
}

mod items;
mod model;
mod receipt;
mod ui;
mod views;

#[derive(Default)]
pub struct App {
    items: Items,
    reciept: Receipt,
    current_model: CurrentModel,
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
            event::KeyCode::Tab => match self.current_model {
                CurrentModel::Items => {
                    self.current_model = CurrentModel::Receipt;
                }
                CurrentModel::Receipt => {
                    self.current_model = CurrentModel::Items;
                }
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyCode;

    use super::*;

    #[test]
    fn test_state_cycling() {
        let mut app = App::default();
        let key_event = event::KeyEvent::from(KeyCode::Tab);

        assert_eq!(
            app.current_model,
            CurrentModel::Items,
            "test if current_view is properly initialized"
        );

        app.handle_key_events(key_event);
        assert_eq!(
            app.current_model,
            CurrentModel::Receipt,
            "test if current view changes with input"
        );

        app.handle_key_events(key_event);
        assert_eq!(
            app.current_model,
            CurrentModel::Items,
            "test if current view can changes back"
        );
    }
}
