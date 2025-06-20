use super::*;

mod interactions_test;
mod render_tests;
mod update_handling;

impl TableData {
    fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 8)
    }
    pub fn mock_items() -> Self {
        let mut table = TableData::new_builder();
        table
            .with_title("Grocery Items")
            .with_heading("Items")
            .with_heading("Price")
            .with_table_type(AppArm::Items);

        let mut table = table.build();
        let items = StoreItem::mock().into_iter().map(DbTable::Item).collect();
        table.add_items(items);

        table
    }

    pub fn mock_receipts() -> TableData {
        let mut table = TableData::new_builder();
        table
            .with_title("Receipt Items")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_heading("Item Qty")
            .with_heading("Payees")
            .with_table_type(AppArm::Receipts);

        let mut table = table.build();

        let items = StoreJoinRow::mock()
            .into_iter()
            .map(DbTable::Receipt)
            .collect();
        table.add_items(items);

        table
    }
}
