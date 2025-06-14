use crate::prelude::*;

pub fn test_home() -> Home {
    let keymap = Config::new().unwrap().keymap.0;

    let mut home = Home::new_builder();

    home.add_component(mock_items_table())
        .add_component(mock_receipts_table())
        .add_key_event_handler(keymap);

    home.build()
}
