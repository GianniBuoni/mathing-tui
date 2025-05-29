use super::*;

pub mod prelude {
    pub use super::{
        DbPayload, DbRequest, DbResponse, ItemParams, ReceiptParams,
        RequestType,
    };
}

mod item_params;

pub trait Request<T> {
    async fn get(&self) -> Result<T>;
    async fn post(&self) -> Result<T>;
    async fn update(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
}

#[derive(Debug, Default)]
pub struct ItemParams {
    item_id: Option<i64>,
    item_name: Option<String>,
    item_price: Option<f64>,
}

#[derive(Debug, Default)]
pub struct ReceiptParams {
    pub id: Option<i64>,
    pub item_id: Option<i64>,
    pub item_qty: Option<i64>,
    pub users: Vec<i64>,
}

pub struct DbResponse {
    pub req_type: RequestType,
    pub payload: DbPayload,
    pub error: Option<String>,
}

pub struct DbRequest {
    pub req_type: RequestType,
    pub payload: DbPayload,
}

pub enum DbPayload {
    ItemParams(ItemParams),
    ReceiptParamss(ReceiptParams),
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
