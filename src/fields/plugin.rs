use super::*;

pub mod prelude {
    pub use super::{new_item_inputs, new_user_inputs};
}

pub fn new_item_inputs(parent: &mut FormBuilder) {
    let Some(DbPayloadBuilder::ItemParams(params)) = &mut parent.payload else {
        return;
    };

    let mut name_input = InputField::<String>::new();
    name_input
        .with_title("Item Name")
        .with_field_type(AppArm::Items);
    let name = name_input.value.clone();

    let mut price_input = InputField::<f64>::new();
    price_input
        .with_title("Item Price")
        .with_field_type(AppArm::Items);
    let price = price_input.value.clone();

    params.item_name(name);
    params.item_price(price);
    name_input.plugin(parent);
    price_input.plugin(parent);
}

pub fn new_user_inputs(parent: &mut FormBuilder) {
    let Some(DbPayloadBuilder::UserParams(params)) = &mut parent.payload else {
        return;
    };

    let mut name_input = InputField::<String>::new();
    name_input
        .with_title("User Name")
        .with_field_type(AppArm::Users);
    let name = name_input.value.clone();

    params.user_name(name);
    name_input.plugin(parent);
}
