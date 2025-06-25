use super::*;
use crate::forms::FormBuilder;

mod outputs;
mod rendering;

pub fn test_text_inputs(parent: &mut FormBuilder) -> Result<()> {
    let Some(DbPayloadBuilder::ItemParams(params)) = &mut parent.payload else {
        let e = FormError::malformed("payload").into();
        return Err(e);
    };
    let string_input = InputField::<String>::test_item_name();
    let name = string_input.value.clone();

    let float_input = InputField::<f64>::test_item_price();
    let price = float_input.value.clone();

    params.item_name(name);
    params.item_price(price);
    string_input.plugin(parent)?;
    float_input.plugin(parent)?;

    Ok(())
}

impl<T> Choice<T>
where
    T: Debug + Default + Copy,
{
    fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 3)
    }
}

impl InputField<f64> {
    fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 3)
    }
    pub fn test_item_price() -> Self {
        let mut input = Self::new();
        input
            .with_title("Item Price")
            .with_field_type(AppArm::Items);
        input
    }
}

impl InputField<String> {
    pub fn test_item_name() -> Self {
        let mut input = Self::new();
        input.with_title("Item Name").with_field_type(AppArm::Items);
        input
    }
}

impl Choice<i64> {
    pub fn test_choice_1() -> Self {
        Self::new("Jon").with_value(1)
    }
    pub fn test_choice_2() -> Self {
        Self::new("Noodle").with_value(1)
    }
    fn plugin_group(parent: &mut SelectionBuilder<i64>) -> Result<()> {
        Self::test_choice_1().plugin(parent)?;
        Self::test_choice_2().plugin(parent)?;
        Ok(())
    }
}

impl SelectionField<i64> {
    pub fn test_rect() -> Rect {
        Rect::new(0, 0, 50, 6)
    }
    pub fn mock() -> Self {
        let mut selection = Self::builder();
        selection
            .with_title("Add Users")
            .with_multselect()
            .add_plugins(Choice::plugin_group)
            .unwrap();

        selection.build().unwrap()
    }
}
