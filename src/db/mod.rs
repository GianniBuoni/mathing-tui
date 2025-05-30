use std::{borrow::Cow, collections::HashMap};

use anyhow::{Error, Result};
use futures::future::try_join_all;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use db_time::get_time;
use queries::prelude::*;

mod connection;
mod db_time;
mod processing;
mod queries;
mod requests;
mod table_displays;
#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::connection::get_db;
    pub use super::queries::prelude::*;
    pub use super::requests::prelude::*;
    #[cfg(test)]
    pub use super::test_cases::*;
    pub use super::{
        ItemParams, ReceiptsUsersParams, StoreItem, StoreJoinRow, StoreTotal,
        StoreUser, UserParams,
    };
}

#[derive(Debug, Default, PartialEq)]
pub struct StoreItem {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
    price: f64,
}

#[derive(Debug, PartialEq)]
pub struct StoreReceipt {
    id: i64,
    created_at: i64,
    updated_at: i64,
    item_id: i64,
    item_qty: i64,
}

#[derive(Debug, PartialEq)]
pub struct StoreJoinRaw {
    item_name: String,
    user_ids: String,
    receipt_id: i64,
    item_id: i64,
    item_price: f64,
    item_qty: i64,
    user_count: i64,
}

#[derive(Debug, PartialEq, Default)]
pub struct StoreJoinRow {
    users: Vec<StoreUser>,
    item_name: String,
    receipt_id: i64,
    item_id: i64,
    item_price: f64,
    item_qty: i64,
    user_count: i64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StoreUser {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
}

#[derive(Debug, Default)]
pub struct StoreTotal(HashMap<i64, Decimal>);

#[derive(Debug, Default)]
pub struct UserParams<'db> {
    u_id: Option<i64>,
    name: Option<Cow<'db, str>>,
}

#[derive(Debug, Default)]
pub struct ItemParams<'db> {
    item_id: Option<i64>,
    item_name: Option<Cow<'db, str>>,
    item_price: Option<f64>,
}

#[derive(Debug, Default)]
pub struct JoinedReceiptParams {
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
    users: Vec<i64>,
}

#[derive(Debug, Default)]
struct ReceiptParams {
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
}

#[derive(Debug, Default)]
pub struct ReceiptsUsersParams {
    r_id: Option<i64>,
    u_id: Option<i64>,
}
