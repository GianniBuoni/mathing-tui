use super::*;

impl PluginInit for TableData {
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.app_index = index;
        self.tracker = tracker;
    }
}

impl Plugin for TableData {
    type Parent = HomeBuilder;

    fn plugin(self, parent: &mut Self::Parent) {
        parent.add_component(self);
    }

    fn plugin_group(parent: &mut Self::Parent) {
        let mut item_table = TableData::new_builder();
        item_table
            .with_title("Store Items")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_table_type(AppArm::Items);
        let item_table = item_table.build();
        item_table.plugin(parent);

        let mut r_table = TableData::new_builder();
        r_table
            .with_title("Receipts")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_heading("Item Qty")
            .with_heading("Payees");
        let r_table = r_table.build();
        r_table.plugin(parent);

        let mut user_table = TableData::new_builder();
        user_table.with_title("Users").with_heading("User Name");
        let user_table = user_table.build();
        user_table.plugin(parent);
    }
}
