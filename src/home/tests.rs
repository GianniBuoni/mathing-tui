use super::*;

#[test]
fn test_handle_events() {
    let key_events = [(KeyEvent::from(KeyCode::Tab), Some(Action::SwitchPane))];

    for (event, want) in key_events {
        let mut home = test_home();
        let got = home.handle_events(Some(Event::Key(event)));
        assert_eq!(
            want, got,
            "Test if default event handlers map to corret actions"
        );
    }
}

#[test]
fn test_component_cycling() {
    let mut test_home = test_home();
    let key_event = KeyEvent::from(KeyCode::Tab);

    assert_eq!(
        *test_home.component_tracker.borrow().deref(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
        test_home.update(action);
        assert_eq!(
            want,
            *test_home.component_tracker.borrow().deref(),
            "Test if current view changes with repeated input"
        );
    }
}

#[test]
fn test_tracker_sync() {
    let mut home = test_home();
    let key_event = KeyEvent::from(KeyCode::Tab);

    for i in 0..100 {
        let action = home.handle_events(Some(Event::Key(key_event)));
        home.update(action);

        let want = if i % 2 == 0 { false } else { true };

        let [item, receipts] = &home.components[..] else {
            panic!("Test case should only have two components.");
        };

        assert_eq!(want, item.is_active(), "Item iteration: {i}");
        assert_eq!(!want, receipts.is_active(), "Receipt iteration: {i}");
    }
}
