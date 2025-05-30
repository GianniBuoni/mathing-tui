use std::borrow::Cow;

use sqlx::SqliteConnection;

use super::*;

pub mod prelude {
    pub use super::errors::RequestError;
    pub use super::{DbPayload, DbRequest, DbResponse, Request, RequestType};
}

mod errors;
mod item_params;
mod user_params;

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
    ) -> impl Future<Output = Result<u64>>;
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
