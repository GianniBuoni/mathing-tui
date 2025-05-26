use super::*;

mod outputs;
mod test_form_rendering;

fn test_big_rect() -> Rect {
    Rect::new(0, 0, 56, 11)
}

fn test_input_rect() -> Rect {
    Rect::new(0, 0, 50, 3)
}

fn test_form<'a>() -> Form<'a> {
    Form::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9))
        .build()
}

fn test_input<'a>() -> InputField<'a, f64> {
    InputField::new("Item Price")
}

fn test_full_form<'a>() -> Form<'a> {
    let name_field = InputField::<String>::new("Item Name");
    let price_field = InputField::<f64>::new("Item Price");

    Form::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9))
        .add_field(name_field)
        .add_field(price_field)
        .build()
}
