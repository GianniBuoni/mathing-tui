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
    Totals,
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

impl TryFrom<&DbPayload> for AppArm {
    type Error = Error;

    fn try_from(value: &DbPayload) -> std::result::Result<Self, Self::Error> {
        match value {
            DbPayload::ItemParams(_) => Ok(Self::Items),
            DbPayload::ReceiptParams(_) => Ok(Self::Receipts),
            DbPayload::UserParams(_) => Ok(Self::Users),
            DbPayload::StoreTotal => Ok(Self::Totals),
            _ => Err(Error::msg(
                "Mapping Error: Only ItemParams and ReceiptParam payloads can be converted to App_Arm.",
            )),
        }
    }
}
