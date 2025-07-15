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
        let got = parse_key_event(raw)?;
        assert_eq!(*want, got, "{desc}");
        Aok(())
    })?;

    Ok(())
}

#[test]
fn test_parse_key_code() -> Result<()> {
    let test_cases = [
        ("esc", KeyCode::Esc),
        ("delete", KeyCode::Delete),
        ("enter", KeyCode::Enter),
        ("tab", KeyCode::Tab),
        ("backspace", KeyCode::Backspace),
        ("left", KeyCode::Left),
        ("down", KeyCode::Down),
        ("up", KeyCode::Up),
        ("right", KeyCode::Right),
        ("a", KeyCode::Char('a')),
        ("b", KeyCode::Char('b')),
        ("c", KeyCode::Char('c')),
    ];

    test_cases.iter().try_for_each(|(raw, want)| {
        let desc = format!("Testing string \"{}\" with no modifiers", raw);
        let got = parse_key_code_add_modifier(raw, KeyModifiers::NONE)?;
        assert_eq!(KeyEvent::from(*want), got, "{desc}");
        Aok(())
    })?;

    Ok(())
}

#[test]
fn test_parse_key_modifiers() {
    let test_cases = [
        ("ctrl-c", ("c", KeyModifiers::CONTROL)),
        ("shift-q", ("q", KeyModifiers::SHIFT)),
        ("ctrl-tab", ("tab", KeyModifiers::CONTROL)),
        ("alt-tab", ("tab", KeyModifiers::ALT)),
    ];

    test_cases.iter().for_each(|(raw, want)| {
        let desc = format!("Test string \"{}\"", raw);
        let got = parse_key_modifiers(raw);
        assert_eq!(*want, got, "{desc}");
    });
}
