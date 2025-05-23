use super::*;

mod test_form_rendering;

fn test_big_rect() -> Rect {
    Rect::new(0, 0, 56, 10)
}

fn test_input_rect() -> Rect {
    Rect::new(0, 0, 50, 3)
}

fn test_form<'a>() -> Form<'a> {
    Form::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 8))
        .build()
}

fn test_input<'a>() -> FormField<'a> {
    FormField::new("Item Name")
}

fn test_full_form<'a>() -> Form<'a> {
    let name_field = FormField::new("Item Name");
    let price_field = FormField::new("Item Price");

    Form::new_builder()
        .add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 8))
        .add_field(name_field)
        .add_field(price_field)
        .build()
}
