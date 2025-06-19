use crate::prelude::*;

impl Home {
    pub fn mock() -> Self {
        let keymap = Config::new().unwrap().keymap.0;

        let mut home = Home::builder();

        home.add_component(TableData::mock_items())
            .add_component(TableData::mock_receipts())
            .add_key_event_handler(keymap);

        home.build()
    }
}
