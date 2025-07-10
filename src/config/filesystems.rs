use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use super::*;

impl AppConfig {
    // Get the currently configured config directory based on the
    // environmental variables.
    pub fn try_get_config_dir() -> Result<PathBuf> {
        let not_found = AppError::config("Couldn't parse system config path.");

        // configured path
        match env::var("MATHING_CONFIG") {
            Ok(configured_path) => Ok(PathBuf::from(configured_path)),
            Err(_) => Ok(dirs::config_dir().ok_or(not_found)?.join("mathing")),
        }
    }

    pub(super) fn check(config_dir: PathBuf) -> Result<(PathBuf, PathBuf)> {
        let config_file = config_dir.join("config.toml");
        let db_file = config_dir.join("data.db");

        // make config dir if not exists
        if !config_dir.exists() {
            let message = "Couldn't create config dir: \"{config_dir}\".";
            create_dir_all(config_dir)
                .map_err(|_| AppError::config(message))?;
        }
        // make and write config file if not exists
        if !(config_file.exists() && config_file.is_file()) {
            let message =
                "Couldn't create/write config file: \"{config_file}\".";
            (|| {
                let mut f = File::create_new(&config_file)?;
                f.write_all(DEFAULT_KEYMAP)
            })()
            .map_err(|_| AppError::config(message))?;
        }
        // make data db if not exists
        if !(db_file.exists() && db_file.is_file()) {
            let message = "Couldn't create app database: \"{db_file}\".";
            File::create_new(&db_file)
                .map_err(|_| AppError::config(message))?;
        }

        Ok((config_file, db_file))
    }
}
