use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use common::*;
use mathing_tui::prelude::*;

mod common;

#[tokio::test]
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
            "Test exiting normal mode.",
        ),
        (
            KeyEvent::from(KeyCode::Enter),
            Some(Action::Submit),
            "Test submitting in normal mode. (Should still retun the submit action)",
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
