use super::*;

mod parsing;

impl AppConfig {
    fn mock_root() -> Result<PathBuf> {
        Ok(PathBuf::from_iter([
            env::var("PWD")?.as_str(),
            ".config",
            "mathing",
        ]))
    }
    fn mock_db_file() -> Result<PathBuf> {
        Ok(Self::mock_root()?.join("data.db"))
    }
    fn mock_keymap_file() -> Result<PathBuf> {
        Ok(Self::mock_root()?.join("config.toml"))
    }
}

#[tokio::test]
async fn test_db_init() -> Result<()> {
    let conn = DbConn::try_init(AppConfig::mock_db_file()?).await;
    assert!(conn.is_ok(), "Test db connection initialization.");

    Ok(())
}

#[test]
fn test_keymap_init() -> Result<()> {
    let keymap = KeyMap::try_init(AppConfig::mock_keymap_file()?);
    assert!(keymap.is_ok(), "Test keymap initialization.");

    Ok(())
}

#[test]
fn test_get_config_dir() -> Result<()> {
    // test different env variables
    assert!(
        false,
        "Test getting the config dir with the different env varaibles."
    );
    Ok(())
}

#[test]
fn test_keymap_builder() -> Result<()> {
    let keymap = KeyMap::try_init(AppConfig::mock_keymap_file()?)?;

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
                let got = keymap.get_action(event).ok_or(Error::msg(
                    format!("Event: {string:?} not found in config keymap."),
                ))?;
                assert_eq!(want, got, "Testing default config for {string}");
            })
        })?;

    Ok(())
}
