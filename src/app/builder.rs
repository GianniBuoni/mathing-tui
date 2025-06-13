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
    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.component.add_component(component);
    }

    pub fn add_to_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        plugin.add_to_app(self);
        self
    }

    pub(super) fn add_plugins(
        &mut self,
        plugin_factory: fn(&mut AppBuilder),
    ) -> &mut Self {
        plugin_factory(self);
        self
    }
}
