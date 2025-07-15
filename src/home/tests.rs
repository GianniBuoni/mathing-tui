use super::*;

impl Home {
    pub fn mock() -> Result<Self> {
        let mut home = Home::builder();
        home.add_component(TableData::mock_items())
            .add_component(TableData::mock_receipts());

        if home.component_tracker.inner() != 0 {
            return Err(Error::msg(
                "Mock home.component_tracker didn't initalize with expected inner value.",
            ));
        }
        home.build()
    }
}

#[test]
fn test_component_cycling_forward() -> Result<()> {
    let mut test_home = Home::mock()?;
    let action = Some(Action::SelectForward);

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        test_home.handle_action(action);
        assert_eq!(
            want,
            test_home.component_tracker.inner(),
            "Test if current view changes with repeated input"
        );
    }
    Ok(())
}

#[test]
fn test_component_cycling_backwards() -> Result<()> {
    let mut test_home = Home::mock()?;
    let action = Some(Action::SelectBackward);

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        test_home.handle_action(action);
        assert_eq!(
            want,
            test_home.component_tracker.inner(),
            "Test if current view changes with repeated input"
        );
    }
    Ok(())
}

#[test]
fn test_tracker_sync() -> Result<()> {
    let mut test_home = Home::mock()?;
    let action = Some(Action::SelectForward);

    for i in 0..100 {
        test_home.handle_action(action);

        let want = i % 2 != 0;

        let [item, receipts] = &test_home.components[..] else {
            panic!("Test case should only have two components.");
        };

        assert_eq!(want, item.is_active(), "Item iteration: {i}");
        assert_eq!(!want, receipts.is_active(), "Receipt iteration: {i}");
    }
    Ok(())
}

#[test]
fn handle_error_response() -> Result<()> {
    let res = DbResponse::new().req_type(RequestType::Update).error(
        RequestError::missing_param(RequestType::Update, "item", "id"),
    );

    let mut home = Home::mock()?;
    home.handle_response(Some(&res))?;

    assert!(home.message.is_some());
    Ok(())
}
