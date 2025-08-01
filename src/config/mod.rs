use std::{
    collections::{BTreeMap, HashMap},
    env,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

use crate::prelude::*;
use keymap::DEFAULT_KEYMAP;
use parsing::*;

pub mod prelude {
    pub use super::{
        AppConfig, CONFIG, ConfigDirs, DbConn, HelpMap, KeyMap, StoreTotal,
    };
}

mod filesystems;
mod helpmap;
mod keymap;
mod parsing;
mod store;
#[cfg(test)]
mod tests;
mod totals;

pub static CONFIG: OnceCell<AppConfig> = OnceCell::const_new();

#[derive(Debug)]
pub struct AppConfig {
    keymap: KeyMap,
    helpmap: HelpMap,
    store: DbConn,
    totals: Mutex<StoreTotal>,
    dirs: ConfigDirs,
}

impl AppConfig {
    /// Initializes all static variables in the app.
    /// Does not return the struct; use the specific getter
    /// for the field instead.
    pub async fn try_init(config_dir: PathBuf) -> Result<()> {
        let config = async || {
            let (keymap_file, db_file) = Self::check(config_dir)?;

            let keymap = KeyMap::try_init(keymap_file.as_path())?;
            let helpmap = HelpMap::try_init(keymap_file.as_path())?;
            let store = DbConn::try_init(db_file.as_path()).await?;
            let totals = StoreTotal::try_init(&store.0).await?;
            let dirs = ConfigDirs::default()
                .with_keymap(keymap_file.as_path())?
                .with_db(db_file.as_path())?;

            Aok(Self {
                keymap,
                helpmap,
                store,
                totals,
                dirs,
            })
        };
        CONFIG.get_or_try_init(config).await?;
        Ok(())
    }
    pub fn try_get_time() -> Result<i64> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
    }
}

#[derive(Default, Debug)]
/// A hashmap of KeyEvent -> Action pairs meant for parsing
/// I/O events during the app's runtime.
pub struct KeyMap(HashMap<KeyEvent, Action>);

#[derive(Debug)]
/// A Btree map of Action -> ActionDictionary pairs
/// and act as a human readable record of which actions
/// are mapped to which keycodes.
pub struct HelpMap(BTreeMap<Action, ActionDictionary>);

#[derive(Debug, Default)]
pub(super) struct ActionDictionary {
    pub(super) raw_keycode: Arc<str>,
    pub(super) descrpition: Arc<str>,
}

/// Stores a string representation of the configured keymap file
/// and the db file locations.
#[derive(Debug, Default, Clone)]
pub struct ConfigDirs {
    pub keymap: Arc<str>,
    pub db: Arc<str>,
}

#[derive(Debug)]
pub struct DbConn(SqlitePool);

#[derive(Debug, Default, PartialEq)]
pub struct StoreTotal(HashMap<i64, Decimal>);
