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

pub fn edit_item_inputs(
    item: &StoreItem,
) -> impl Fn(&mut FormBuilder) -> Result<()> {
    move |parent| {
        let params = try_get_item_params(parent)?;

        let mut name_input = InputField::<String>::new();
        name_input
            .with_title("Item Name")
            .with_field_type(AppArm::Items)
            .with_default_value(item.name.clone());
        let name = name_input.value.clone();

        let mut price_input = InputField::<f64>::new();
        price_input
            .with_title("Item Price")
            .with_field_type(AppArm::Items)
            .with_default_value(item.price);
        let price = price_input.value.clone();

        params.item_id(ParamOption::new().map_value(item.id).to_owned());
        params.item_name(name);
        params.item_price(price);
        name_input.plugin(parent)?;
        price_input.plugin(parent)?;

        Ok(())
    }
}
