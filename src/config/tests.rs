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
                "Testing {}; assumes using POSIX file format",
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
    ];

    test_cases.iter().try_for_each(|(raw, want, desc)| {
        Ok::<(), String>({
            let got = parse_key_event(raw)?;
            assert_eq!(*want, got, "{desc}");
        })
    })?;

    Ok(())
}
