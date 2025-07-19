use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use common::prelude::*;
use mathing_tui::prelude::*;

mod common;

#[tokio::test]
/// Test basic key -> action mapping from the keymap
async fn test_handle_key_events() -> Result<()> {
    try_init_test_config().await?;

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
            KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT),
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
            "Test exiting to normal mode.",
        ),
        (
            KeyEvent::from(KeyCode::Enter),
            Some(Action::Submit),
            "Test submit.",
        ),
        (
            KeyEvent::from(KeyCode::Char('j')),
            Some(Action::NavigateDown),
            "Test navigating table down with j.",
        ),
        (
            KeyEvent::from(KeyCode::Down),
            Some(Action::NavigateDown),
            "Test navigating table down with DOWN.",
        ),
        (
            KeyEvent::from(KeyCode::Char('k')),
            Some(Action::NavigateUp),
            "Test navigating table up with k.",
        ),
        (
            KeyEvent::from(KeyCode::Up),
            Some(Action::NavigateUp),
            "Test navigating table up with UP.",
        ),
    ];
    let keymap = KeyMap::get()
        .ok_or(Error::msg("Keymap didn't initialize correctly."))?;

    key_events.into_iter().for_each(|(key, want, desc)| {
        let got = keymap.get_action(key);
        assert_eq!(want, got, "{desc}");
    });

    Ok(())
}

// test key events in a form w/ an input
#[tokio::test]
async fn test_form_handle_key_events() -> Result<()> {
    try_init_test_config().await?;

    let test_cases = [
        (
            KeyEvent::from(KeyCode::Char('i')),
            Some(Action::HandleInput(KeyEvent::from(KeyCode::Char('i')))),
            "Test handle input.",
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
            KeyEvent::from(KeyCode::Enter),
            Some(Action::Submit),
            "Test submitting.",
        ),
    ];

    let form = Form::new_item()?;
    test_cases.into_iter().for_each(|(event, want, desc)| {
        let got = form.handle_key_events(event);
        assert_eq!(want, got, "{desc}")
    });
    Ok(())
}
