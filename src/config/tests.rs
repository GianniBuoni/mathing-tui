use super::*;

#[test]
fn test_parse_key_event() -> Result<()> {
    let test_cases = [
        (
            "ctrl-c",
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            "Test lowercase string",
        ),
        (
            "CTRL-C",
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            "Test all caps string",
        ),
        (
            "Ctrl-C",
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            "Test variable capitalization",
        ),
        (
            "alt-tab",
            KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT),
            "Test modifier and non-char keycode",
        ),
    ];

    test_cases.iter().try_for_each(|(raw, want, desc)| {
        Aok::<()>({
            let got = parse_key_event(raw)?;
            assert_eq!(*want, got, "{desc}");
        })
    })?;

    Ok(())
}

#[test]
fn test_config_builder() -> Result<()> {
    let config = Config::get_config();

    let test_cases = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Quit,
            "c",
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

    test_cases
        .into_iter()
        .try_for_each(|(event, want, string)| {
            Aok({
                let got = config.get(event).ok_or(Error::msg(format!(
                    "Event: {string:?} not found in config keymap."
                )))?;
                assert_eq!(want, got, "Testing default config for {string}");
            })
        })?;

    Ok(())
}
