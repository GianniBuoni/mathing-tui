use super::*;

mod outputs;
mod test_form_rendering;

fn test_big_rect() -> Rect {
    Rect::new(0, 0, 56, Form::TWO_FIELD_H)
}

fn test_input_rect() -> Rect {
    Rect::new(0, 0, 50, 3)
}

fn test_form() -> Form {
    let mut form = Form::new_builder();
    form.add_title("Add New Item").add_rect(Rect::new(
        0,
        0,
        52,
        Form::TWO_FIELD_H,
    ));
    form.build()
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

fn test_valid_form(source: &OutputStruct) -> Form {
    let name_field =
        InputField::<String>::new("Item Name").map_value(source.name.clone());
    let price_field =
        InputField::<f64>::new("Item Price").map_value(source.price.clone());

    let mut form = Form::new_builder();
    form.add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 52, 12))
        .add_field(name_field)
        .add_field(price_field)
        .add_request_type(RequestType::Post);
    form.build()
}

fn test_invalid_form_no_fields() -> Form {
    let mut form = Form::new_builder();
    form.add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9));
    form.build()
}
