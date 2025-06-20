use super::*;

pub mod prelude {
    pub use super::{new_item_inputs, new_user_inputs};
}

pub fn new_item_inputs(parent: &mut FormBuilder) -> Result<()> {
    let Some(form_type) = &parent.form_type else {
        let e = FormErrors::malformed("form type").into();
        return Err(e);
    };
    if !(*form_type == AppArm::Items) {
        let e = FormErrors::mapping(AppArm::Users, *form_type).into();
        return Err(e);
    }
    let Some(DbPayloadBuilder::ItemParams(params)) = &mut parent.payload else {
        let e = FormErrors::malformed("payload").into();
        return Err(e);
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
    name_input.plugin(parent)?;
    price_input.plugin(parent)?;

    Ok(())
}

pub fn new_user_inputs(parent: &mut FormBuilder) -> Result<()> {
    let Some(form_type) = &parent.form_type else {
        let e = FormErrors::malformed("form type").into();
        return Err(e);
    };
    if !(*form_type == AppArm::Users) {
        let e = FormErrors::mapping(AppArm::Users, *form_type).into();
        return Err(e);
    }
    let Some(DbPayloadBuilder::UserParams(params)) = &mut parent.payload else {
        let e = FormErrors::malformed("payload").into();
        return Err(e);
    };

    let mut name_input = InputField::<String>::new();
    name_input
        .with_title("User Name")
        .with_field_type(AppArm::Users);
    let name = name_input.value.clone();

    params.user_name(name);
    name_input.plugin(parent)?;

    Ok(())
}
