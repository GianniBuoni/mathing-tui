use super::*;

#[test]
fn test_key_events() {
    Config::get_config();
    let home = Home::mock();

    let key_events = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Some(Action::Quit),
            "Test default quit event.",
        ),
        (
            KeyEvent::from(KeyCode::Tab),
            Some(Action::SelectForward),
            "Test default pane switch.",
        ),
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT),
            Some(Action::SelectBackward),
            "Test pane switch backwards",
        ),
        (
            KeyEvent::from(KeyCode::Char('i')),
            Some(Action::EnterInsert),
            "Test entering insert mode.",
        ),
        (
            KeyEvent::from(KeyCode::Esc),
            Some(Action::EnterNormal),
            "Test exiting normal mode.",
        ),
        (
            KeyEvent::from(KeyCode::Enter),
            Some(Action::Submit),
            "Test submitting in normal mode. (Should still retun the submit action)",
        ),
        (
            KeyEvent::from(KeyCode::Char('j')),
            Some(Action::TableNavigateDown),
            "Test navigating table down with j.",
        ),
        (
            KeyEvent::from(KeyCode::Down),
            Some(Action::TableNavigateDown),
            "Test navigating table down with DOWN.",
        ),
        (
            KeyEvent::from(KeyCode::Char('k')),
            Some(Action::TableNavigateUp),
            "Test navigating table up with k.",
        ),
        (
            KeyEvent::from(KeyCode::Up),
            Some(Action::TableNavigateUp),
            "Test navigating table up with UP.",
        ),
    ];

    key_events.into_iter().for_each(|(event, want, desc)| {
        let got = home.handle_key_events(event);
        assert_eq!(want, got, "{desc}")
    });
}

#[test]
fn test_component_cycling_forward() {
    let mut test_home = Home::mock();
    let key_event = KeyEvent::from(KeyCode::Tab);

    assert_eq!(
        test_home.component_tracker.inner(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
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
    let key_event = KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT);

    assert_eq!(
        test_home.component_tracker.inner(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
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
    let mut home = Home::mock();
    let key_event = KeyEvent::from(KeyCode::Tab);

    for i in 0..100 {
        let action = home.handle_events(Some(Event::Key(key_event)));
        home.handle_action(action);

        let want = if i % 2 == 0 { false } else { true };

        let [item, receipts] = &home.components[..] else {
            panic!("Test case should only have two components.");
        };

        assert_eq!(want, item.is_active(), "Item iteration: {i}");
        assert_eq!(!want, receipts.is_active(), "Receipt iteration: {i}");
    }
}
