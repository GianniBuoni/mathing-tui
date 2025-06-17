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

    let user_table = TableData::<StoreUser>::new_builder()
        .add_title("Users")
        .add_heading("User Name")
        .build();

    app.add_component(TableTui::Items(item_table))
        .add_component(TableTui::Receipt(r_table))
        .add_component(TableTui::Users(user_table));
}
