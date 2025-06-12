use super::*;

mod outputs;
mod test_form_rendering;

fn test_big_rect() -> Rect {
    Rect::new(0, 0, 56, 11)
}

fn test_input_rect() -> Rect {
    Rect::new(0, 0, 50, 3)
}

fn test_form() -> Form<ItemParams> {
    Form::<ItemParams>::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9))
        .build()
}

fn test_f64_input<'a>() -> InputField<f64> {
    InputField::new("Item Price")
}

fn test_str_input<'a>() -> InputField<String> {
    InputField::new("Item Name")
}

#[derive(Default)]
struct OutputStruct {
    name: Rc<RefCell<String>>,
    price: Rc<RefCell<f64>>,
}

fn test_valid_form(source: &OutputStruct) -> Form<ItemParams> {
    let name_field =
        InputField::<String>::new("Item Name").map_value(source.name.clone());
    let price_field =
        InputField::<f64>::new("Item Price").map_value(source.price.clone());

    Form::<ItemParams>::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9))
        .add_field(name_field)
        .add_field(price_field)
        .build()
}

fn test_invalid_form_no_fields() -> Form<ItemParams> {
    Form::<ItemParams>::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9))
        .build()
}
