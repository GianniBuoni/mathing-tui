use std::{collections::HashMap, env, path::PathBuf};

use crate::prelude::*;
use anyhow::Result;

pub mod prelude {
    pub use super::Config;
}

const DEFAULT_CONFIG_PATH: [&str; 3] = [".config", "mathing", "config.toml"];

#[derive(Default, Debug)]
pub struct Config {
    pub keymap: KeyMap,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct KeyMap(HashMap<KeyEvent, Action>);

pub fn config_dir() -> Result<PathBuf> {
    let path = match env::var("PLATFORM")?.as_ref() {
        "development" => env::var("PWD")?,
        _ => env::var("HOME")?,
    };

    let mut path = PathBuf::from(path);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_dir() -> Result<()> {
        assert!(
            config_dir()?
                .to_string_lossy()
                .contains("/.config/mathing/config.toml"),
            "Test assumes that no custom path has been set."
        );
        Ok(())
    }
}
