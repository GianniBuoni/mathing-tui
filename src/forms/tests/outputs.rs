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
fn test_form_validation() -> Result<()> {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('1'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('.'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
        Action::HandleInput(KeyEvent::from(KeyCode::Char('9'))),
    ];

    let mut form = test_full_form();
    key_events.iter().for_each(|key| form.update(Some(*key)));

    let form = FormTui::ItemForm(form);
    form.validate()?;
    Ok(())
}

#[test]
fn test_invalid_data() {
    let key_events = [
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
        Action::SelectForward,
        Action::HandleInput(KeyEvent::from(KeyCode::Char('a'))),
    ];

    let mut form = test_full_form();
    key_events.iter().for_each(|key| form.update(Some(*key)));

    let form = FormTui::ItemForm(form);
    match form.validate() {
        Ok(_) => panic!("expected an Error"),
        Err(got) => {
            let want = "Item price is not a float.";
            assert_eq!(want, AsRef::<str>::as_ref(&got.to_string()))
        }
    }
}
