use std::{collections::HashMap, error::Error};

use futures::future::try_join_all;
use rust_decimal::prelude::*;
use sqlx::SqlitePool;

use db_time::get_time;
use queries::prelude::*;

mod connection;
mod db_time;
mod processing;
mod queries;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::connection::get_db;
    pub use super::queries::prelude::*;
    pub use super::{StoreItem, StoreJoinRow, StoreTotal, StoreUser};
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct StoreJoinRow {
    users: Vec<StoreUser>,
    item_name: String,
    receipt_id: i64,
    item_id: i64,
    item_price: f64,
    item_qty: i64,
    user_count: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StoreUser {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
}

#[derive(Debug, Default)]
pub struct StoreTotal(HashMap<i64, Decimal>);
