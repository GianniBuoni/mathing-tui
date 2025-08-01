use super::*;

mod parsing;

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
