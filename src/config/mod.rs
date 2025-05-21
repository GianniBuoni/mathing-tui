use std::collections::HashMap;

use crate::prelude::*;

pub mod prelude {
    pub use super::Config;
    pub use super::filesystems::{config_check, config_dir};
}

mod filesystems;
#[cfg(test)]
mod tests;

const DEFAULT_CONFIG_PATH: [&str; 2] = ["mathing", "config.toml"];

#[derive(Default, Debug)]
pub struct Config {
    pub keymap: KeyMap,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct KeyMap(HashMap<KeyEvent, Action>);
