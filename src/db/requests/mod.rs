use std::borrow::Cow;

use sqlx::SqliteConnection;

use super::*;

pub mod prelude {
    pub use super::errors::RequestError;
    pub use super::{
        DbPayload, DbRequest, DbResponse, ItemParams, JoinedReceiptParams,
        Request, RequestType,
    };
}

mod errors;
mod item_params;

pub trait Request<T> {
    fn check_id(&self) -> Result<i64>;

    fn get(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl Future<Output = Result<T>>;

    fn post(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl Future<Output = Result<T>>;

    fn update(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl Future<Output = Result<T>>;

    fn delete(
        &self,
        conn: &mut SqliteConnection,
    ) -> impl Future<Output = Result<()>>;
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
