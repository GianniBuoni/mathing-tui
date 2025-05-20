use crate::prelude::*;

pub fn test_home<'a>() -> Home<'a> {
    Home::new_builder()
        .add_component(TableTui::Items(mock_items()))
        .add_component(TableTui::Receipt(mock_receipts()))
        .build()
}
