use super::*;

impl PluginInit for TableData {
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.app_index = index;
        self.tracker = tracker;
    }
}

impl Plugin for TableData {
    type Parent = HomeBuilder;

    fn plugin(self, parent: &mut Self::Parent) -> Result<()> {
        parent.add_component(self);
        Ok(())
    }

    fn plugin_group(parent: &mut Self::Parent) -> Result<()> {
        let mut item_table = TableData::builder();
        item_table
            .with_title("Store Items")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_table_type(AppArm::Items);
        let item_table = item_table.build()?;

        let mut r_table = TableData::builder();
        r_table
            .with_title("Receipts")
            .with_heading("Item Name")
            .with_heading("Item Price")
            .with_heading("Item Qty")
            .with_heading("Payees")
            .with_table_type(AppArm::Receipts);
        let r_table = r_table.build()?;

        let mut user_table = TableData::builder();
        user_table
            .with_title("Users")
            .with_heading("User Name")
            .with_heading("Totals")
            .with_table_type(AppArm::Users);
        let user_table = user_table.build()?;

        item_table.plugin(parent)?;
        r_table.plugin(parent)?;
        user_table.plugin(parent)?;

        Ok(())
    }
}
