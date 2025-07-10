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
    pub fn mock_keymap_file() -> Result<PathBuf> {
        Ok(Self::mock_root()?.join("config.toml"))
    }
}

#[test]
fn test_keymap_init() -> Result<()> {
    let keymap = KeyMap::try_init(AppConfig::mock_keymap_file()?);
    assert!(keymap.is_ok(), "Test keymap initialization.");

    Ok(())
}

#[test]
fn test_get_config_dir() -> Result<()> {
    let configured = PathBuf::from(env::var("PWD")?).join("mathing");

    temp_env::with_vars(
        [
            ("MATHING_CONFIG", Some(&configured.to_str().unwrap())),
            ("MATHING_CONFIG", None),
        ],
        || {
            let got = AppConfig::try_get_config_dir()?;
            let want = match env::var("MATHING_CONFIG") {
                Ok(_) => &configured,
                Err(_) => &dirs::config_dir().unwrap().join("mathing"),
            };
            assert_eq!(*want, got, "Test if config file changes with env.");
            Aok(())
        },
    )
}
