use sqlx::prelude::FromRow;

use super::*;

pub mod prelude {
    pub use super::{DbTable, StoreItem, StoreJoinRow, StoreUser};
}

mod db_table;
mod store_join_row;

// public structs
#[derive(Debug, Clone)]
pub enum DbTable {
    Item(StoreItem),
    User(StoreUser),
    Receipt(StoreJoinRow),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StoreUser {
    pub id: i64,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, FromRow)]
pub struct StoreItem {
    pub id: i64,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StoreJoinRow {
    pub users: Vec<StoreUser>,
    pub item_name: String,
    pub(super) user_count: i64,
    pub receipt_id: i64,
    pub(super) item_id: i64,
    pub(super) item_price: f64,
    pub item_qty: i64,
}

// "private" structs. These should onlys be used within
// the db module.
#[derive(Debug, Default)]
pub(super) struct StoreCount {
    pub(super) rows: i64,
}

#[derive(Debug, PartialEq)]
pub(super) struct StoreReceipt {
    pub(super) id: i64,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
    pub(super) item_id: i64,
    pub(super) item_qty: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct StoreReceiptsUsers {
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
    pub(super) receipt_id: i64,
    pub(super) user_id: i64,
}

#[derive(Debug, PartialEq, FromRow)]
pub(super) struct StoreJoinPrices {
    pub(super) user_ids: String,
    pub(super) user_count: i64,
    pub(super) item_price: f64,
    pub(super) item_qty: i64,
}

#[derive(Debug, PartialEq, FromRow)]
pub(super) struct StoreJoinRaw {
    pub(super) item_name: String,
    pub(super) user_ids: String,
    pub(super) receipt_id: i64,
    pub(super) item_id: i64,
    pub(super) item_price: f64,
    pub(super) item_qty: i64,
    pub(super) user_count: i64,
}
