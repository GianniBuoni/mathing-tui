use super::*;

#[test]
fn test_input_validation() {
    let key_events = [
        (
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('1')))),
            "Ok",
            "Test valid input.",
        ),
        (
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('a')))),
            "Unable to parse \"a\" as f64.",
            "Test invalid input.",
        ),
        (None, "Item Price is unset.", "Test unset data."),
    ];

    key_events.iter().for_each(|(action, want, desc)| {
        let mut float_input = test_input();

        float_input.update(*action);
        let got = match float_input.validate() {
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
