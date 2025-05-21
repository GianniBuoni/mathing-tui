use std::{
    collections::HashMap,
    env,
    fs::{File, create_dir_all},
    io::{Error, ErrorKind},
    path::PathBuf,
};

use crate::prelude::*;
use anyhow::Result;

pub mod prelude {
    pub use super::Config;
}

const DEFAULT_CONFIG_PATH: [&str; 2] = ["mathing", "config.toml"];

#[derive(Default, Debug)]
pub struct Config {
    pub keymap: KeyMap,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct KeyMap(HashMap<KeyEvent, Action>);

pub fn config_dir() -> Result<PathBuf> {
    let mut path = match env::var("PLATFORM")?.as_ref() {
        "development" => PathBuf::from(env::var("PWD")?).join(".config"),
        _ => dirs::config_dir().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidFilename,
                "Could not parse config filepath for this platform.",
            )
        })?,
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

pub fn config_exists() -> Result<bool> {
    Ok(config_dir()?.exists() && config_dir()?.is_file())
}

pub fn config_check() -> Result<()> {
    if !config_exists()? {
        let path = config_dir()?;

        create_dir_all(path.parent().ok_or_else(|| {
            Error::new(
                ErrorKind::NotADirectory,
                "Could not parse parent dir for config file.",
            )
        })?)?;

        File::create_new(path)?;
        //TODO: add logging feature to report that file was created.
    }

    Ok(())
}

#[cfg(test)]
mod tests {
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
}
