use super::*;

pub(super) fn parse_key_event(raw: &str) -> Result<KeyEvent, AppError> {
    let raw_lower = raw.to_ascii_lowercase();
    let (keycode, modifiers) = parse_key_modifiers(&raw_lower);
    parse_key_code_add_modifier(keycode, modifiers)
}

fn parse_key_modifiers(raw: &str) -> (&str, KeyModifiers) {
    let modifier = match raw {
        str if str.starts_with("shift-") => KeyModifiers::SHIFT,
        str if str.starts_with("ctrl-") => KeyModifiers::CONTROL,
        str if str.starts_with("alt-") => KeyModifiers::ALT,
        _ => KeyModifiers::NONE,
    };
    let remaining = raw.split("-").last().unwrap_or(raw);

    (remaining, modifier)
}

fn parse_key_code_add_modifier(
    raw: &str,
    modifiers: KeyModifiers,
) -> Result<KeyEvent, AppError> {
    let message = format!("unable to parse {raw}; invalid or unsupported");
    let err = AppError::config(message);

    let code = match raw {
        "esc" => KeyCode::Esc,
        "delete" => KeyCode::Delete,
        "enter" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backspace" => KeyCode::Backspace,
        "left" => KeyCode::Left,
        "down" => KeyCode::Down,
        "up" => KeyCode::Up,
        "right" => KeyCode::Right,
        s if s.len() == 1 => {
            let c = s.chars().next().ok_or(err)?;
            KeyCode::Char(c)
        }
        _ => {
            return Err(err);
        }
    };

    Ok(KeyEvent::new(code, modifiers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_event() -> Result<()> {
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
            Aok::<()>({
                let desc =
                    format!("Testing string \"{}\" with no modifiers", raw);
                let got = parse_key_code_add_modifier(raw, KeyModifiers::NONE)?;
                assert_eq!(KeyEvent::from(*want), got, "{desc}");
            })
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
}
