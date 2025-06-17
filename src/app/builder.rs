use super::*;

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
    pub fn add_component(&mut self, component: TableTui) -> &mut Self {
        self.component.add_component(component);
        self
    }
}

impl PluginParent for AppBuilder {}
