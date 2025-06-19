use super::*;

impl PluginInit for Home {
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        let _ = index;
        let _ = tracker;
        todo!();
    }
}

impl Plugin for Home {
    type Parent = AppBuilder;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.component = self
    }

    fn plugin_group(parent: &mut Self::Parent) {
        let mut home = Self::builder();

        let Ok(keymap) = Config::new() else {
            home.build().error =
                Some("Config could not be made or parsed".to_string());
            return;
        };

        home.add_key_event_handler(keymap.keymap.0)
            .add_request_handler(parent.tui.req_tx.clone())
            .add_plugins(TableData::plugin_group);

        home.build().plugin(parent);
    }
}
