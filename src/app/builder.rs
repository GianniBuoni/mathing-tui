use crate::home::HomeBuilder;

use super::*;

#[derive(Debug, Default)]
pub struct AppBuilder {
    pub component: HomeBuilder,
    pub tui: TuiBuilder,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }
}

impl AppBuilder {
    pub fn build(mut self) -> App {
        self.component.add_request_handler(self.tui.req_tx.clone());

        App {
            component: self.component.build(),
            should_exit: false,
            tui: self.tui.build(),
        }
    }

    pub fn add_home(&mut self, home: HomeBuilder) -> &mut Self {
        self.component = home;
        self
    }
}
