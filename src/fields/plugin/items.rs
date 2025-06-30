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

    params.with_item_name(name);
    params.with_item_price(price);
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

        params.with_item_id(ParamOption::new().map_value(item.id).to_owned());
        params.with_item_name(name);
        params.with_item_price(price);
        name_input.plugin(parent)?;
        price_input.plugin(parent)?;

        Ok(())
    }
}

pub fn search_item_imputs(parent: &mut FormBuilder) -> Result<()> {
    let params = try_get_item_params(parent)?;

    let mut search_input = InputField::<String>::new();
    search_input
        .with_field_type(AppArm::Items)
        .with_title("Search for Item");
    let search_term = search_input.value.clone();

    params.with_search(search_term);
    search_input.plugin(parent)?;

    Ok(())
}
