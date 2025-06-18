use super::*;

mod choice_rendering;
mod form_rendering;
mod input_rendering;
mod outputs;

fn test_big_rect() -> Rect {
    Rect::new(0, 0, 56, Form::TWO_FIELD_H)
}

fn test_input_rect() -> Rect {
    Rect::new(0, 0, 50, 3)
}

fn test_form() -> Form {
    let mut form = Form::new();
    form.add_title("Add New Item").add_rect(Rect::new(
        0,
        0,
        52,
        Form::TWO_FIELD_H,
    ));
    form
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

fn test_valid_form() -> FormTui {
    let mut form = Form::new();
    form.add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 52, 12))
        .add_request_type(RequestType::Post);
    let mut form = FormTui::ItemForm(form);

    InputField::<String>::new("Item Name").plugin(&mut form);
    InputField::<f64>::new("Item Price").plugin(&mut form);

    form.map(|f| f.init())
}

fn test_invalid_form_no_fields() -> Form {
    let mut form = Form::new();
    form.add_title("Add New Item")
        .add_rect(Rect::new(0, 0, 50, 9));
    form
}
