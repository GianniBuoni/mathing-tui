use std::collections::HashMap;

use serde::Deserialize;

use crate::prelude::*;

pub mod prelude {
    pub use super::Config;
    pub use super::filesystems::{config_check, config_dir};
    pub use super::parsing::parse_key_event;
}

mod builder;
mod filesystems;
mod parsing;
#[cfg(test)]
mod tests;

const DEFAULT_CONFIG_PATH: [&str; 2] = ["mathing", "config.toml"];

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keymap: KeyMap,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct KeyMap(pub HashMap<KeyEvent, Action>);
