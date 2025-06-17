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

    app.add_to_plugin(TableTui::Items(item_table))
        .add_to_plugin(TableTui::Receipt(r_table))
        .add_to_plugin(TableTui::Users(user_table));
}

impl Plugin for TableTui {
    type Parent = AppBuilder;

    fn add_to_parent(self, app: &mut Self::Parent) {
        match self {
            Self::Items(_) => app.add_component(self),
            Self::Receipt(_) => app.add_component(self),
            Self::Users(_) => app.add_component(self),
        }
    }
}
