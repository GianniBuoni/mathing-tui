use std::ops::Deref;

use super::*;

#[test]
fn test_input_validation() {
    let mut key_events = [
        (
            Box::new(test_f64_input()) as Box<dyn Field>,
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('1')))),
            "Ok",
            "Test valid float input.",
        ),
        (
            Box::new(test_f64_input()) as Box<dyn Field>,
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('a')))),
            "Unable to parse \"a\" as f64.",
            "Test invalid input.",
        ),
        (
            Box::new(test_f64_input()) as Box<dyn Field>,
            None,
            "Item Price is unset.",
            "Test unset data.",
        ),
        (
            Box::new(test_str_input()) as Box<dyn Field>,
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('a')))),
            "Ok",
            "Test valid str input.",
        ),
    ];

    key_events
        .iter_mut()
        .for_each(|(input, action, want, desc)| {
            input.update(*action);
            let got = match input.validate() {
                Ok(_) => "Ok".to_string(),
                Err(e) => e.to_string(),
            };

            assert_eq!(want.to_string(), got, "{desc}")
        });
}

#[test]
fn test_input_mapping() {
    let test_float = Rc::new(RefCell::new(0. as f64));

    let mut test_cases = [
        (
            Box::new(test_f64_input().map_value(test_float.clone()))
                as Box<dyn Field>,
            "Ok",
            "Test valid float field.",
        ),
        (
            Box::new(test_f64_input()) as Box<dyn Field>,
            "Item Price is not mapped to any value.",
            "Test unmapped float field.",
        ),
    ];

    test_cases.iter_mut().for_each(|(input, want, desc)| {
        let action =
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))));
        input.update(action);

        let got = match input.submit() {
            Ok(_) => "Ok".to_string(),
            Err(e) => e.to_string(),
        };

        assert_eq!(want.to_string(), got, "{desc}")
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

    let mut form = test_valid_form(&OutputStruct::default());
    key_events.iter().for_each(|key| form.update(Some(*key)));
    form.fields.iter().try_for_each(|field| field.validate())?;

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

    let got = OutputStruct::default();
    let mut form = test_valid_form(&got);
    key_events.iter().for_each(|key| form.update(Some(*key)));
    form.submit()?;

    let name = got.name.borrow();
    let got = (name.deref().deref(), *got.price.borrow());

    assert_eq!(want, got, "Test if valid form creates expected output");

    Ok(())
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
    key_events.iter().for_each(|key| form.update(Some(*key)));

    let want = "Malformed: form has no fields.".to_string();
    let got = match form.submit() {
        Ok(_) => panic!("Expected an error!"),
        Err(e) => e.to_string(),
    };

    assert_eq!(want, got, "Test malformed form");
}
