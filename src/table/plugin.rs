use super::*;

pub(crate) fn plugin(app: &mut AppBuilder) {
    let item_table = TableData::<StoreItem>::new_builder()
        .add_title("Store Items")
        .add_heading("Item Name")
        .add_heading("Item Price")
        .build();

    let r_table = TableData::<StoreJoinRow>::new_builder()
        .add_title("Receipts")
        .add_heading("Item Name")
        .add_heading("Item Price")
        .add_heading("Item Qty")
        .add_heading("Payees")
        .build();

    app.add_to_plugin(item_table).add_to_plugin(r_table);
}

impl<T> Plugin for TableData<T> where T: TableDisplay + 'static {}
