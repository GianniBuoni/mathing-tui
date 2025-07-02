use std::array;

use super::*;

mod interactions_test;
mod render_tests;
mod update_handling;

impl TableData {
    fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 8)
    }
    pub fn mock_items() -> Self {
        let mut table = TableData::builder();
        table
            .with_title("Grocery Items")
            .with_heading("Items")
            .with_heading("Price")
            .with_table_type(AppArm::Items);

        let mut table = table.build().unwrap();
        let items = DbPayload::Items(StoreItem::mock().into());
        table.add_items(items);

        table
    }

    pub fn mock_receipts() -> TableData {
        let mut table = TableData::builder();
        table
            .with_title("Receipt Items")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_heading("Item Qty")
            .with_heading("Payees")
            .with_table_type(AppArm::Receipts);

        let mut table = table.build().unwrap();
        let items = DbPayload::Receipts(StoreJoinRow::mock().into());
        table.add_items(items);

        table
    }

    fn mock_pages() -> [Self; 3] {
        let mut init: [Self; 3] = array::from_fn(|_| Self::default());
        // edit counts and limits
        init[0].count = 55;
        init[0].limit = 20;
        init[0].pages = 3;
        init[1].count = 40;
        init[1].limit = 20;
        init[1].pages = 2;
        init[2].count = 5;
        init[2].limit = 13;
        init[2].pages = 1;

        init
    }
}
