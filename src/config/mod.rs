use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::sync::OnceCell;

use crate::prelude::*;
use keymap::DEFAULT_KEYMAP;

pub mod prelude {
    pub use super::{AppConfig, DbConn, KeyMap, StoreTotal};
}

mod filesystems;
mod keymap;
mod store;
#[cfg(test)]
mod tests;
mod totals;

static CONFIG: OnceCell<AppConfig> = OnceCell::const_new();

#[derive(Debug)]
pub struct AppConfig {
    keymap: KeyMap,
    store: DbConn,
    totals: Mutex<StoreTotal>,
}

impl AppConfig {
    /// Initializes all static variables in the app.
    /// Does not return the struct; use the specific getter
    /// for the field instead.
    pub async fn try_init() -> Result<()> {
        let config = async || {
            let (config_dir, db_dir) = Self::check()?;

            let keymap = KeyMap::try_init(config_dir)?;
            let store = DbConn::try_init(db_dir).await?;
            let totals = StoreTotal::try_init(&store.0).await?;

            Aok(Self {
                keymap,
                store,
                totals,
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
pub struct KeyMap(HashMap<KeyEvent, Action>);

#[derive(Debug)]
pub struct DbConn(SqlitePool);

#[derive(Debug, Default, PartialEq)]
pub struct StoreTotal(HashMap<i64, Decimal>);
