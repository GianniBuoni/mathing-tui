use temp_env::with_vars;

use super::*;

#[test]
fn test_config_dir() -> Result<()> {
    with_vars(
        [
            ("PLATFORM", Some("development")),
            ("PLATFORM", Some("production")),
        ],
        || {
            let got = config_dir()
                .expect("config_dir function retuned unexpected error");

            assert!(
                got.to_string_lossy()
                    .contains("/.config/mathing/config.toml"),
                "Testing {}; assumes using POSIX file path",
                got.to_string_lossy()
            );
        },
    );

    Ok(())
}

#[test]
fn test_parse_key_event() -> Result<(), String> {
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
            "Shift-tab",
            KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT),
            "Test shift modifier and non-char keycode",
        ),
    ];

    test_cases.iter().try_for_each(|(raw, want, desc)| {
        Ok::<(), String>({
            let got = parse_key_event(raw)?;
            assert_eq!(*want, got, "{desc}");
        })
    })?;

    Ok(())
}

#[test]
fn test_config_builder() -> Result<()> {
    let test_cases = [
        (
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            Action::Quit,
            "c",
        ),
        (KeyEvent::from(KeyCode::Tab), Action::SelectForward, "tab"),
        (KeyEvent::from(KeyCode::Esc), Action::EnterNormal, "esc"),
        (KeyEvent::from(KeyCode::Char('i')), Action::EnterInsert, "i"),
        (
            KeyEvent::from(KeyCode::Char('j')),
            Action::TableNavigateDown,
            "j",
        ),
        (
            KeyEvent::from(KeyCode::Down),
            Action::TableNavigateDown,
            "down",
        ),
        (
            KeyEvent::from(KeyCode::Char('k')),
            Action::TableNavigateUp,
            "k",
        ),
        (KeyEvent::from(KeyCode::Up), Action::TableNavigateUp, "up"),
    ];

    let config = Config::new()?;

    test_cases.iter().for_each(|(event, want, string)| {
        let got = config.keymap.0.get(event).unwrap();
        assert_eq!(want, got, "Testing default config for {string}");
    });

    Ok(())
}
