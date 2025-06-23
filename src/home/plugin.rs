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

    fn plugin(self, parent: &mut Self::Parent) -> Result<()> {
        parent.component = self;
        Ok(())
    }

    fn plugin_group(parent: &mut Self::Parent) -> Result<()> {
        let mut home = Self::builder();

        home.add_request_handler(parent.tui.req_tx.clone())
            .add_plugins(TableData::plugin_group)?;

        home.build()?.plugin(parent)?;

        Ok(())
    }
}
