use super::*;

pub fn new_item_inputs(parent: &mut FormBuilder) -> Result<()> {
    let params = try_get_item_params(parent)?;

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
    name_input.plugin(parent)?;
    price_input.plugin(parent)?;

    Ok(())
}
