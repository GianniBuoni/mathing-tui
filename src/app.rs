use std::collections::HashMap;

use crate::prelude::*;
use ratatui::DefaultTerminal;

impl Default for App {
    fn default() -> Self {
        let mut models = HashMap::new();

        let items = Box::new(Items::default());
        let reciept = Box::new(Receipt::default());

        models.insert(CurrentModel::Items, items as Box<dyn Model>);
        models.insert(CurrentModel::Receipt, reciept as Box<dyn Model>);

        Self {
            models,
            current_model: CurrentModel::default(),
            should_exit: bool::default(),
        }
    }
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

    pub fn list_models(&self) -> Vec<&Box<dyn Model>> {
        let mut models: Vec<&Box<dyn Model>> = self.models.values().collect();
        models.sort_by_key(|f| f.index());
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
