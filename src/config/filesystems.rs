use std::{
    env,
    fs::{File, create_dir_all},
    io::{Error, ErrorKind, Write},
    path::PathBuf,
};

use super::*;

impl Config {
    fn config_dir() -> Result<PathBuf> {
        let notfound_error = Error::new(
            ErrorKind::NotFound,
            "Could not parse config filepath for this platform.",
        );

        let mut path = match env::var("PLATFORM") {
            Ok(str) => match str {
                str if str == "development" => {
                    PathBuf::from(env::var("PWD")?).join(".config")
                }
                _ => dirs::config_dir().ok_or(notfound_error)?,
            },
            Err(_) => dirs::config_dir().ok_or(notfound_error)?,
        };

        match env::var("MATHING_CONFIG") {
            Ok(dir) => {
                path = path.join(dir).join("config.toml");
            }
            Err(_) => DEFAULT_CONFIG_PATH.iter().for_each(|s| {
                path = path.join(s);
            }),
        }

        Ok(path)
    }

    pub(super) fn check() -> Result<PathBuf> {
        let path = Self::config_dir()?;
        let config_exists = path.exists() && path.is_file();

        if !config_exists {
            create_dir_all(path.parent().ok_or_else(|| {
                anyhow::Error::msg(
                    "Could not parse parent dir for config file.",
                )
            })?)?;

            let mut f = File::create_new(path.clone())?;
            f.write_all(DEFAULT_CONFIG)?;
            //TODO: add logging feature to report that file was created.
        }

        Ok(path)
    }
}
