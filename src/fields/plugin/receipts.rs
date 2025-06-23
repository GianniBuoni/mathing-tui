use super::*;

pub fn new_receipt_inputs_middleware(
    item: &StoreItem,
    users: Vec<&StoreUser>,
) -> impl Fn(&mut FormBuilder) -> Result<()> {
    move |parent| {
        let params = try_get_receipt_params(parent)?;

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
