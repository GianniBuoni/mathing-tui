use super::*;

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }
    pub fn new() -> Self {
        let mut app = App::builder();
        app.add_plugins(Home::plugin_group);
        app.build()
    }
}

impl PluginParent for AppBuilder {}

impl ComponentBuilder for AppBuilder {
    type Output = App;

    fn build(self) -> Self::Output {
        App {
            component: self.component,
            should_exit: false,
            tui: self.tui.build(),
        }
    }
}
