#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use sqlx::SqlitePool;
use std::error::Error;

mod connection;
mod db_time;
mod join_tables;
mod queries;
#[cfg(test)]
mod tests;

pub(self) use connection::get_db;
pub(self) use db_time::get_time;
pub(self) use queries::prelude::*;

pub struct StoreItem {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
    price: f64,
}

pub struct StoreReceipt {
    id: i64,
    created_at: i64,
    updated_at: i64,
    item_id: i64,
    item_qty: i64,
}

pub(self) struct StoreReceiptsUsers {
    created_at: i64,
    updated_at: i64,
    receipt_id: i64,
    user_id: i64,
}

#[derive(Clone)]
pub(self) struct StoreJoinRow {
    item: String,
    payee: String,
    item_qty: i64,
    payees_total: i64,
    price: f64,
}

pub struct StoreJoinTable {
    payees: Vec<String>,
    item: String,
    item_qty: i64,
    price: f64,
}

pub struct StoreUser {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
}
