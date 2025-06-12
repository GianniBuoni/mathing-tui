use crate::prelude::*;

pub mod prelude {
    pub use super::App;
}

mod builder;

#[derive(Debug)]
pub struct App {
    component: Home,
    tui: Tui,
    should_exit: bool,
}

impl App {
    pub async fn run(&mut self) -> Result<()> {
        while !self.should_exit {
            let _ = self.tui.next_response();
            let event = self.tui.next_event().await;

            let action = self.handle_events(event);
            //handle responses

            self.update(action);

            self.tui
                .terminal
                .draw(|frame| self.component.draw(frame, frame.area()))?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
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
            Some(_) => {
                self.component.update(action);
            }
            None => {}
        }
    }
}
