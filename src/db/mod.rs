use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fmt::{Debug, Display},
    ops::Deref,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use futures::future::try_join_all;
use rust_decimal::prelude::*;
use sqlx::{SqliteConnection, SqliteExecutor, SqlitePool};
use tokio::sync::OnceCell;

use crate::prelude::*;
use tables::{StoreJoinRaw, StoreReceipt, StoreReceiptsUsers};

mod params;
mod payloads;
mod requests;
mod response;
mod tables;
#[cfg(test)]
mod tests;
mod totals;

pub mod prelude {
    pub use super::DbConn;
    pub use super::params::prelude::*;
    pub use super::payloads::prelude::*;
    pub use super::requests::prelude::*;
    pub use super::response::prelude::*;
    pub use super::tables::prelude::*;
    pub use super::totals::prelude::*;
    // TODO: remove later when ifgure out how to make param building
    // a consuming operation?
    pub use super::params::{
        items::ItemParamsBuilder, join_row::JoinParamsBuilder,
        users::UserParamsBuilder,
    };
}

static DB: OnceCell<DbConn> = OnceCell::const_new();

pub struct DbConn(SqlitePool);

impl DbConn {
    // TODO: gets db_url from Config
    async fn new() -> Result<Self> {
        let db_string = env::var("DATABASE_URL")?;
        Ok(Self(SqlitePool::connect(&db_string).await?))
    }
    pub async fn try_get() -> Result<&'static SqlitePool> {
        let conn = DB.get_or_try_init(Self::new).await?;
        Ok(&conn.0)
    }
    pub fn try_get_time() -> Result<i64> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64)
    }
}
