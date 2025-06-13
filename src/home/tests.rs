use super::*;

#[test]
fn test_key_events() {
    let mut app = test_home();

    let none = KeyModifiers::NONE;
    let key_events = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Some(Action::Quit),
            "Test default quit event.",
        ),
        (
            KeyEvent::new(KeyCode::Tab, none),
            Some(Action::SelectForward),
            "Test default pane switch.",
        ),
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT),
            Some(Action::SelectBackward),
            "Test pane switch backwards",
        ),
        (
            KeyEvent::new(KeyCode::Char('i'), none),
            Some(Action::EnterInsert),
            "Test entering insert mode.",
        ),
        (
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::SHIFT),
            Some(Action::HandleInput(KeyEvent::new(
                KeyCode::Char('i'),
                KeyModifiers::SHIFT,
            ))),
            "Test entering input in insert mode.",
        ),
        (
            KeyEvent::new(KeyCode::Enter, none),
            Some(Action::Submit),
            "Test submitting in edit mode.",
        ),
        (
            KeyEvent::new(KeyCode::Esc, none),
            Some(Action::EnterNormal),
            "Test exiting normal mode.",
        ),
        (
            KeyEvent::new(KeyCode::Enter, none),
            Some(Action::Submit),
            "Test submitting in normal mode. (Should still retun the submit action)",
        ),
        (
            KeyEvent::new(KeyCode::Char('j'), none),
            Some(Action::TableNavigateDown),
            "Test navigating table down with j.",
        ),
        (
            KeyEvent::new(KeyCode::Down, none),
            Some(Action::TableNavigateDown),
            "Test navigating table down with DOWN.",
        ),
        (
            KeyEvent::new(KeyCode::Char('k'), none),
            Some(Action::TableNavigateUp),
            "Test navigating table up with k.",
        ),
        (
            KeyEvent::new(KeyCode::Up, none),
            Some(Action::TableNavigateUp),
            "Test navigating table up with UP.",
        ),
    ];

    key_events.into_iter().for_each(|(event, want, desc)| {
        let got = app.handle_events(Some(Event::Key(event)));
        app.update(got, None);
        assert_eq!(want, got, "{desc}")
    });
}

#[test]
fn test_component_cycling_forward() {
    let mut test_home = test_home();
    let key_event = KeyEvent::from(KeyCode::Tab);

    assert_eq!(
        *test_home.component_tracker.borrow(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
        test_home.update(action, None);
        assert_eq!(
            want,
            *test_home.component_tracker.borrow(),
            "Test if current view changes with repeated input"
        );
    }
}

#[test]
fn test_component_cycling_backwards() {
    let mut test_home = test_home();
    let key_event = KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT);

    assert_eq!(
        *test_home.component_tracker.borrow(),
        0,
        "Test if current model is properly initialized",
    );

    for i in 0..100 {
        let want = if i % 2 == 0 { 1 } else { 0 };

        let action = test_home.handle_events(Some(Event::Key(key_event)));
        test_home.update(action, None);
        assert_eq!(
            want,
            *test_home.component_tracker.borrow(),
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
        home.update(action, None);

        let want = if i % 2 == 0 { false } else { true };

        let [item, receipts] = &home.components[..] else {
            panic!("Test case should only have two components.");
        };

        assert_eq!(want, item.is_active(), "Item iteration: {i}");
        assert_eq!(!want, receipts.is_active(), "Receipt iteration: {i}");
    }
}
