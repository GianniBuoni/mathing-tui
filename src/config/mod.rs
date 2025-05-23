use std::{collections::HashMap, sync::Once};

use serde::Deserialize;

use crate::prelude::*;

pub mod prelude {
    pub use super::Config;
    pub use super::filesystems::{config_check_once, config_dir};
    pub use super::parsing::parse_key_event;
}

mod builder;
mod filesystems;
mod parsing;
#[cfg(test)]
mod tests;

const DEFAULT_CONFIG_PATH: [&str; 2] = ["mathing", "config.toml"];

const DEFAULT_CONFIG: &[u8; 193] = b"[keymap]
\"CTRL-c\" = \"Quit\"
\"TAB\" = \"SwitchPane\"
\"ESC\" = \"EnterNormal\"
\"i\" = \"EnterInsert\"
\"j\" = \"TableNavigateDown\"
\"DOWN\" = \"TableNavigateDown\"
\"k\" = \"TableNavigateUp\"
\"UP\" = \"TableNavigateUp\"";

static CONFIG_CHECK: Once = Once::new();

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keymap: KeyMap,
}

#[derive(Default, Debug)]
pub struct KeyMap(pub HashMap<KeyEvent, Action>);
