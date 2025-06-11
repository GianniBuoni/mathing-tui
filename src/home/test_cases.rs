use crate::prelude::*;

pub fn test_home() -> Home {
    let keymap = Config::new().unwrap().keymap.0;

    Home::new_builder()
        .add_component(TableTui::Items(mock_items()))
        .add_component(TableTui::Receipt(mock_receipts()))
        .add_key_event_handler(keymap)
        .build()
}
