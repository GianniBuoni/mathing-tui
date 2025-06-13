use crate::prelude::*;

pub mod prelude {
    pub use super::{App, AppBuilder};
}

mod builder;
mod plugin;

#[derive(Debug)]
pub struct App {
    component: Home,
    tui: Tui,
    should_exit: bool,
}

#[derive(Default)]
pub struct AppBuilder {
    pub component: HomeBuilder,
    pub tui: TuiBuilder,
}

impl App {
    pub async fn run(&mut self) -> Result<()> {
        while !self.should_exit {
            let res = self.tui.next_response();
            let event = self.tui.next_event().await;

            let action = self.handle_events(event);

            self.update(action, res.as_ref());

            self.tui
                .terminal
                .draw(|frame| self.component.draw(frame, frame.area()))?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Quit) => Some(Action::Quit),
            Some(Event::Key(_)) => self.component.handle_events(event),
            Some(_) => None,
            None => None,
        }
    }

    pub fn update(
        &mut self,
        action: Option<Action>,
        response: Option<&DbResponse>,
    ) {
        match action {
            Some(Action::Quit) => {
                self.should_exit = true;
            }
            Some(_) => {
                self.component.update(action, response);
            }
            None => {}
        }
    }
}
