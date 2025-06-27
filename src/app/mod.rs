use crate::prelude::*;

pub mod prelude {
    pub use super::{App, AppArm, AppBuilder};
}

mod builder;
mod component;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AppArm {
    Items,
    Receipts,
    Users,
}

#[derive(Debug)]
pub struct App {
    component: Home,
    tui: Tui,
    should_exit: bool,
}

#[derive(Default)]
pub struct AppBuilder {
    pub component: Home,
    pub tui: TuiBuilder,
}

impl App {
    pub async fn run(&mut self) -> Result<()> {
        while !self.should_exit {
            let res = self.tui.next_response();
            let event = self.tui.next_event();

            let action = self.handle_events(event);

            self.handle_action(action);
            self.handle_response(res.as_ref())?;

            self.tui
                .terminal
                .draw(|frame| self.component.draw(frame, frame.area()))?;
        }
        Ok(())
    }
}
