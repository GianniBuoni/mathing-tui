use core::panic;

use super::*;

#[test]
fn test_input_validation_f64() {
    let mut key_events = [
        (
            test_f64_input(),
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('1')))),
            None,
            "Test valid float input.",
        ),
        (
            test_f64_input(),
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('a')))),
            Some(FormErrors::validation("a", "f64").to_string()),
            "Test invalid input.",
        ),
        (
            test_f64_input(),
            None,
            Some(FormErrors::no_data("Item Price").to_string()),
            "Test unset data.",
        ),
    ];

    key_events
        .iter_mut()
        .for_each(|(input, action, want, desc)| {
            input.handle_action(*action);
            let got = input.validate().map_err(|e| e.to_string()).err();
            assert_eq!(*want, got, "{desc}")
        });
}

#[test]
fn test_form_validation() -> Result<()> {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let mut form = test_valid_form();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));
    form.try_mut_inner(|f| f.submit())?;

    Ok(())
}

#[test]
fn test_form_submit() -> Result<()> {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let want = ("a", 1.99 as f64);

    let mut form = test_valid_form();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));
    form.try_mut_inner(|f| f.submit())?;

    // check if from values changed
    let panic_msg = "Test is not testing the expected kind of form.";
    let desc = "Test if submitting with input values produces the correct resulting value.";

    match form {
        FormTui::ItemForm(form) => {
            if let DbPayloadBuilder::ItemParams(params) = form.payload.unwrap()
            {
                let name = params.item_name.unwrap().unwrap();
                let price = params.item_price.unwrap().unwrap();

                assert_eq!(want.0, name, "{desc}");
                assert_eq!(want.1, price, "{desc}");

                Ok(())
            } else {
                panic!("{panic_msg}")
            }
        }
        _ => panic!("{panic_msg}"),
    }
}

#[test]
fn test_malformed_form_error() {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let mut form = test_invalid_form_no_fields();
    key_events
        .iter()
        .for_each(|key| form.handle_action(Some(*key)));

    let want = "Malformed: form has no fields.".to_string();
    let got = match form.submit() {
        Ok(_) => panic!("Expected an error!"),
        Err(e) => e.to_string(),
    };

    assert_eq!(want, got, "Test malformed form");
}
