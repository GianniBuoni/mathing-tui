use super::*;

pub(super) fn parse_key_event(raw: &str) -> Result<KeyEvent, AppError> {
    let raw_lower = raw.to_ascii_lowercase();
    let (keycode, modifiers) = parse_key_modifiers(&raw_lower);
    parse_key_code_add_modifier(keycode, modifiers)
}

pub(super) fn parse_key_modifiers(raw: &str) -> (&str, KeyModifiers) {
    let modifier = match raw {
        str if str.starts_with("shift-") => KeyModifiers::SHIFT,
        str if str.starts_with("ctrl-") => KeyModifiers::CONTROL,
        str if str.starts_with("alt-") => KeyModifiers::ALT,
        _ => KeyModifiers::NONE,
    };
    let remaining = raw.split("-").last().unwrap_or(raw);

    (remaining, modifier)
}

pub(super) fn parse_key_code_add_modifier(
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
