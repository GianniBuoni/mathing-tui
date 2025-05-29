use std::borrow::Cow;

use super::*;

pub mod prelude {
    pub use super::{
        DbPayload, DbRequest, DbResponse, ItemParams, JoinedReceiptParams,
        RequestType,
    };
}

mod item_params;

pub trait Request<T> {
    async fn get(&self) -> Result<T>;
    async fn post(&self) -> Result<T>;
    async fn update(&self) -> Result<T>;
    async fn delete(&self) -> Result<T>;
}

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

pub struct DbResponse<'db> {
    pub req_type: RequestType,
    pub payload: DbPayload<'db>,
    pub error: Option<String>,
}

pub struct DbRequest<'db> {
    pub req_type: RequestType,
    pub payload: DbPayload<'db>,
}

pub enum DbPayload<'db> {
    ItemParams(ItemParams<'db>),
    ReceiptParams(JoinedReceiptParams),
    Item(StoreItem),
    Receipt(StoreJoinRow),
}

#[derive(Clone, Copy)]
pub enum RequestType {
    Get,
    GetAll,
    Post,
    Update,
    Delete,
    DeleteAll,
}
