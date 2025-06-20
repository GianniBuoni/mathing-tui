use super::*;

#[test]
fn test_input_validation_f64() {
    let mut key_events = [
        (
            InputField::<f64>::test_item_price(),
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('1')))),
            None,
            "Test valid float input.",
        ),
        (
            InputField::<f64>::test_item_price(),
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('a')))),
            Some(FormErrors::validation("a", "f64").to_string()),
            "Test invalid input.",
        ),
        (
            InputField::<f64>::test_item_price(),
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
