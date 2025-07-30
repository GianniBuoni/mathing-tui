use common::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

mod common;

#[tokio::test]
async fn test_db_init() -> Result<()> {
    try_init_test_config().await?;
    let got = DbConn::try_get();

    assert!(
        got.is_ok(),
        "Test if db connection is initalized and can be retrived"
    );
    Ok(())
}

#[tokio::test]
async fn test_keymap_builder() -> Result<()> {
    try_init_test_config().await?;

    let test_cases = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Quit,
            "ctrl-c",
        ),
        (KeyEvent::from(KeyCode::Tab), Action::SelectForward, "tab"),
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT),
            Action::SelectBackward,
            "alt-tab",
        ),
        (KeyEvent::from(KeyCode::Esc), Action::EnterNormal, "esc"),
        (KeyEvent::from(KeyCode::Char('i')), Action::EnterInsert, "i"),
        (
            KeyEvent::from(KeyCode::Char('j')),
            Action::NavigateDown,
            "j",
        ),
        (KeyEvent::from(KeyCode::Down), Action::NavigateDown, "down"),
        (KeyEvent::from(KeyCode::Char('k')), Action::NavigateUp, "k"),
        (KeyEvent::from(KeyCode::Up), Action::NavigateUp, "up"),
    ];

    let keymap = KeyMap::get()
        .ok_or(Error::msg("Couldn't get keymap form the config."))?;

    test_cases
        .into_iter()
        .try_for_each(|(event, want, string)| {
            let message = format!("Couldn't find {event:?} in keymap.");
            let got = keymap.get_action(event).ok_or(Error::msg(message))?;
            assert_eq!(
                want, got,
                "Test if default config has action for {string}"
            );
            Aok(())
        })?;

    Ok(())
}

#[tokio::test]
async fn test_getting_raw_key_strings() -> Result<()> {
    try_init_test_config().await?;

    let test_cases = [
        (Action::MakeSelection, "SPACE"),
        (Action::EnterNormal, "ESC"),
        (Action::Help, "?"),
        (Action::SelectBackward, "ALT-TAB"),
        (Action::Reset, "CTRL-r"),
        (Action::NavigateLeft, "h, LEFT"),
        (Action::NavigateDown, "j, DOWN"),
        (Action::NavigateUp, "k, UP"),
        (Action::NavigateRight, "l, RIGHT"),
    ];

    let helpmap = HelpMap::get()
        .ok_or(Error::msg("Couldn't get keymap form the config."))?;

    test_cases.into_iter().try_for_each(|(event, want)| {
        let message = format!("Couldn't find {event:?} in helpmap.");
        let got = helpmap.get_key_str(event).ok_or(Error::msg(message))?;
        assert_eq!(
            want, got,
            "Test if default conifg has correct key string for {event:?}."
        );
        Aok(())
    })?;

    Ok(())
}
