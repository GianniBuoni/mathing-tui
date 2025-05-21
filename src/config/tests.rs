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
