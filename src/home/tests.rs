use super::*;

impl Home {
    pub fn mock() -> Self {
        let mut home = Home::builder();

        home.add_component(TableData::mock_items())
            .add_component(TableData::mock_receipts());

        home.build().unwrap()
    }
}

#[test]
fn test_component_cycling_forward() {
    let mut test_home = Home::mock();
    let action = Some(Action::SelectForward);

    assert_eq!(
        test_home.component_tracker.inner(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        test_home.handle_action(action);
        assert_eq!(
            want,
            test_home.component_tracker.inner(),
            "Test if current view changes with repeated input"
        );
    }
}

#[test]
fn test_component_cycling_backwards() {
    let mut test_home = Home::mock();
    let action = Some(Action::SelectBackward);

    assert_eq!(
        test_home.component_tracker.inner(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        test_home.handle_action(action);
        assert_eq!(
            want,
            test_home.component_tracker.inner(),
            "Test if current view changes with repeated input"
        );
    }
}

#[test]
fn test_tracker_sync() {
    let mut test_home = Home::mock();
    let action = Some(Action::SelectForward);

    for i in 0..100 {
        test_home.handle_action(action);

        let want = if i % 2 == 0 { false } else { true };

        let [item, receipts] = &test_home.components[..] else {
            panic!("Test case should only have two components.");
        };

        assert_eq!(want, item.is_active(), "Item iteration: {i}");
        assert_eq!(!want, receipts.is_active(), "Receipt iteration: {i}");
    }
}

#[test]
fn handle_error_response() -> Result<()> {
    let res = DbResponse::new().req_type(RequestType::Update).error(
        RequestError::missing_param(RequestType::Update, "item", "id"),
    );

    let mut home = Home::mock();
    home.handle_response(Some(&res))?;

    assert!(home.message.is_some());
    Ok(())
}
