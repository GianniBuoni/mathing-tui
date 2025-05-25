use crate::prelude::*;

pub fn test_app<'a>() -> App<'a> {
    App::new(test_home())
}

#[test]
fn test_input_propogation() {
    let none = KeyModifiers::NONE;
    let key_events = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Quit,
            "Test default quit event.",
        ),
        (
            KeyEvent::new(KeyCode::Tab, none),
            Action::SelectForward,
            "Test default pane switch.",
        ),
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT),
            Action::SelectBackward,
            "Test pane switch backwards",
        ),
        (
            KeyEvent::new(KeyCode::Char('i'), none),
            Action::EnterInsert,
            "Test entering insert mode.",
        ),
        (
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::SHIFT),
            Action::HandleInput(KeyEvent::new(
                KeyCode::Char('i'),
                KeyModifiers::SHIFT,
            )),
            "Test entering input in insert mode.",
        ),
        (
            KeyEvent::new(KeyCode::Esc, none),
            Action::EnterNormal,
            "Test entering normal mode.",
        ),
        (
            KeyEvent::new(KeyCode::Char('j'), none),
            Action::TableNavigateDown,
            "Test navigating table down with j.",
        ),
        (
            KeyEvent::new(KeyCode::Down, none),
            Action::TableNavigateDown,
            "Test navigating table down with DOWN.",
        ),
        (
            KeyEvent::new(KeyCode::Char('k'), none),
            Action::TableNavigateUp,
            "Test navigating table up with k.",
        ),
        (
            KeyEvent::new(KeyCode::Up, none),
            Action::TableNavigateUp,
            "Test navigating table up with UP.",
        ),
    ];

    let mut app = test_app();
    key_events.into_iter().for_each(|(event, want, desc)| {
        let got = app.handle_events(Some(Event::Key(event)));
        app.update(got);
        assert_eq!(Some(want), got, "{desc}")
    });
}
