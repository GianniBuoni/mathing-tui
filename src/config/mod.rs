use std::{collections::HashMap, sync::OnceLock};

use serde::Deserialize;

use crate::prelude::*;
use parsing::parse_key_event;

pub mod prelude {
    pub use super::Config;
}

mod builder;
mod filesystems;
mod parsing;
#[cfg(test)]
mod tests;

const DEFAULT_CONFIG_PATH: [&str; 2] = ["mathing", "config.toml"];

const DEFAULT_CONFIG: &[u8; 421] = b"[keymap]
\"CTRL-c\" = \"Quit\"
\"a\" = \"AddToReceipt\"
\"d\" = \"DeleteSelected\"
\"e\" = \"EditSelected\"
\"i\" = \"EnterInsert\"
\"ESC\" = \"EnterNormal\"
\" \" = \"MakeSelection\"
\"LEFT\" = \"NavigateLeft\"
\"h\" = \"NavigateLeft\"
\"DOWN\" = \"NavigateDown\"
\"j\" = \"NavigateDown\"
\"UP\" = \"NavigateUp\"
\"k\" = \"NavigateUp\"
\"RIGHT\" = \"NavigateRight\"
\"l\" = \"NavigateRight\"
\"TAB\" = \"SelectForward\"
\"SHIFT-TAB\" = \"SelectBackward\"
\"y\" = \"Submit\"
\"ENTER\" = \"Submit\"";

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    keymap: KeyMap,
}

#[derive(Default, Debug)]
pub struct KeyMap(pub HashMap<KeyEvent, Action>);

impl Config {
    pub fn get(&self, key: KeyEvent) -> Option<Action> {
        self.keymap.0.get(&key).copied()
    }
}
