use crate::prelude::*;

impl Home {
    pub fn mock() -> Self {
        let mut home = Home::builder();

        home.add_component(TableData::mock_items())
            .add_component(TableData::mock_receipts());

        home.build().unwrap()
    }
}
