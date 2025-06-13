use super::*;

impl App {
    pub fn init() -> Result<AppBuilder> {
        let mut app = App::builder();
        let keymap = Config::new()?.keymap.0;

        app.component.add_key_event_handler(keymap);
        app.add_plugins(table_plugin);

        Ok(app)
    }
}
