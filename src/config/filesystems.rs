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
        let config_file = config_dir.join("keymap.toml");
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

impl ConfigDirs {
    fn parse_path(path: &Path) -> Result<Arc<str>> {
        let res = path
            .to_str()
            .ok_or_else(|| {
                let message =
                    format!("Couldn't parse \"{path:?}\" as a string.");
                AppError::config(message)
            })?
            .into();

        Ok(res)
    }
    pub(super) fn with_keymap(mut self, keymap_file: &Path) -> Result<Self> {
        self.keymap = Self::parse_path(keymap_file)?;
        Ok(self)
    }
    pub(super) fn with_db(mut self, db_file: &Path) -> Result<Self> {
        self.db = Self::parse_path(db_file)?;
        Ok(self)
    }
    pub fn get() -> Result<Self> {
        Ok(CONFIG.get().ok_or(AppError::ConfigInit)?.dirs.clone())
    }
}
