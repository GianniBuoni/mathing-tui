use sqlx::SqliteExecutor;

use super::*;

pub mod prelude {
    pub use super::errors::RequestError;
    pub use super::{DbPayload, DbRequest, DbResponse, Request, RequestType};
}

mod builders;
mod errors;
mod item_params;
mod joined_params;
mod receipts_params;
mod receipts_users_params;
mod user_params;

pub trait Request<'e> {
    type Output;
    type Connection: SqliteExecutor<'e>;

    fn check_id(&self) -> Result<i64>;
    fn get(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn get_all(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Vec<Self::Output>>>;
    fn post(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn update(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn delete(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<u64>>;
}

#[derive(Debug)]
pub struct DbResponse<'db> {
    pub req_type: RequestType,
    pub payload: DbPayload<'db>,
    pub error: Option<String>,
}

pub struct DbRequest<'db> {
    pub req_type: RequestType,
    pub payload: DbPayload<'db>,
}

#[derive(Debug, Default)]
pub enum DbPayload<'db> {
    #[default]
    None,
    ItemParams(ItemParams<'db>),
    ReceiptParams(JoinedReceiptParams),
    Item(StoreItem),
    Items(Vec<StoreUser>),
    Receipt(StoreJoinRow),
    Receipts(Vec<StoreJoinRow>),
    User(StoreUser),
}

#[derive(Clone, Copy, Debug)]
pub enum RequestType {
    Get,
    GetAll,
    Post,
    Update,
    Delete,
    DeleteAll,
}
