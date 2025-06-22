use super::*;

pub mod prelude {
    pub use super::{
        new_item_inputs, new_receipt_inputs_middleware, new_user_inputs,
    };
}

pub fn new_item_inputs(parent: &mut FormBuilder) -> Result<()> {
    let Some(form_type) = &parent.form_type else {
        let e = FormErrors::malformed("form type").into();
        return Err(e);
    };
    if !(*form_type == AppArm::Items) {
        let e = FormErrors::mapping(AppArm::Items, *form_type).into();
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

pub fn new_receipt_inputs_middleware(
    item: &StoreItem,
    users: Vec<&StoreUser>,
) -> impl Fn(&mut FormBuilder) -> Result<()> {
    move |parent| {
        let Some(form_type) = &parent.form_type else {
            let e = FormErrors::malformed("form type").into();
            return Err(e);
        };
        if !(*form_type == AppArm::Receipts) {
            let e = FormErrors::mapping(AppArm::Receipts, *form_type).into();
            return Err(e);
        }
        let Some(DbPayloadBuilder::ReceiptParams(params)) = &mut parent.payload
        else {
            let e = FormErrors::malformed("payload").into();
            return Err(e);
        };

        let mut qty_input = InputField::<i64>::new();
        qty_input
            .with_title("Item Qty")
            .with_field_type(AppArm::Receipts);
        let item_qty = qty_input.value.clone();

        let mut user_select = SelectionField::<i64>::builder();
        user_select.with_title("Add Users").with_multselect();
        users
            .iter()
            .map(|f| Choice::<i64>::new(f.name.clone()).with_value(f.id))
            .try_for_each(|f| f.plugin(&mut user_select))?;
        let user_select = user_select.build()?;
        let users = user_select.values.clone();

        params.item_id(ParamOption::new().map_value(item.id).to_owned());
        params.item_qty(item_qty);
        params.users(users);

        qty_input.plugin(parent)?;
        user_select.plugin(parent)?;

        Ok(())
    }
}
