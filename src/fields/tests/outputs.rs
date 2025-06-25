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

#[test]
fn test_get_rect_height() {
    let test_case = SelectionField::mock();
    let got = test_case.get_rect_height();
    assert_eq!(6, got, "Test rect height calculations.");
}

#[test]
fn test_multi_selection_submit() {
    let make_selection = Some(Action::MakeSelection);
    let next_choice = Some(Action::NavigateDown);

    let test_cases = [
        (
            vec![make_selection],
            "OK".to_string(),
            "Test single selection.",
        ),
        (
            vec![make_selection, next_choice, make_selection],
            "OK".to_string(),
            "Test multi selection.",
        ),
        (
            vec![None],
            FormErrors::no_data("choices").to_string(),
            "Test no selection.",
        ),
    ];

    test_cases.into_iter().for_each(|(actions, want, desc)| {
        let mut test_selection = SelectionField::mock();
        actions
            .into_iter()
            .for_each(|action| test_selection.handle_action(action));
        let got = match test_selection.submit() {
            Ok(_) => "OK".to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(want, got, "{desc}");
    });
}

#[test]
fn test_single_selection_submit() {
    let make_selection = Some(Action::MakeSelection);
    let next_choice = Some(Action::NavigateDown);

    let test_cases = [
        (
            vec![make_selection],
            "OK".to_string(),
            "Test single selection.",
        ),
        (
            vec![make_selection, next_choice, make_selection],
            FormErrors::validation("multi-select", "single select").to_string(),
            "Test multi selection.",
        ),
        (
            vec![None],
            FormErrors::no_data("choices").to_string(),
            "Test no selection.",
        ),
    ];

    test_cases.into_iter().for_each(|(actions, want, desc)| {
        let mut test_selection = SelectionField::mock();
        test_selection.multiselect = false;

        actions
            .into_iter()
            .for_each(|action| test_selection.handle_action(action));
        let got = match test_selection.submit() {
            Ok(_) => "OK".to_string(),
            Err(e) => e.to_string(),
        };
        assert_eq!(want, got, "{desc}");
    });
}
