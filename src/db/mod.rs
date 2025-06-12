use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use anyhow::{Error, Result};
use futures::future::try_join_all;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use db_time::get_time;

mod connection;
mod db_time;
mod processing;
mod requests;
mod table_displays;
#[cfg(test)]
mod test_cases;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::connection::get_db;
    pub use super::requests::prelude::*;
    #[cfg(test)]
    pub use super::test_cases::*;
    pub use super::{
        ItemParams, JoinedReceiptParams, StoreItem, StoreJoinRow, StoreTotal,
        StoreUser, UserParams,
    };
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StoreUser {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
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

#[derive(Debug, PartialEq, Eq)]
struct StoreReceiptsUsers {
    created_at: i64,
    updated_at: i64,
    receipt_id: i64,
    user_id: i64,
}

#[derive(Debug, PartialEq)]
struct StoreJoinRaw {
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
    user_count: i64,
    receipt_id: i64,
    item_id: i64,
    item_price: f64,
    item_qty: i64,
}

#[derive(Debug, Default)]
pub struct StoreTotal(HashMap<i64, Decimal>);

#[derive(Debug, Default, PartialEq)]
pub struct UserParams {
    u_id: Option<i64>,
    name: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct ItemParams {
    item_id: Option<i64>,
    item_name: Option<String>,
    item_price: Option<f64>,
    offset: Option<i64>,
}

#[derive(Debug, Default, PartialEq)]
pub struct JoinedReceiptParams {
    users: Vec<i64>,
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Default)]
struct ReceiptsUsersParams {
    r_id: Option<i64>,
    u_id: Option<i64>,
}

#[derive(Debug, Default)]
struct ReceiptParams {
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
}
