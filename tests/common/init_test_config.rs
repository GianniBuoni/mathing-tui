use super::*;

/// Sets up the testing CONFIG static struct.
/// Assures that the test does not depend on whether
/// or not certain app env variables are set.
/// For keymap testing only; use [`try_init_test_db`]
/// for database integration tests.
pub async fn try_init_test_config() -> Result<()> {
    let config_dir =
        PathBuf::from_iter([env::var("PWD")?.as_str(), ".config", "mathing"]);
    AppConfig::try_init(config_dir).await?;

    Ok(())
}
