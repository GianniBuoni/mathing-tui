use std::ops::Deref;

use errors::RequestError;
use sqlx::SqliteExecutor;

use super::*;

pub mod prelude {
    pub use super::errors::RequestError;
    pub use super::handle_requests::handle_requests;
    pub use super::{
        DbPayload, DbPayloadBuilder, DbRequest, DbResponse, ItemParamsBuilder,
        JoinParamsBuilder, ParamOption, Request, RequestType,
        UserParamsBuilder,
    };
}

mod builders;
mod errors;
mod handle_requests;
mod item_params;
mod joined_params;
mod payloads;
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

#[derive(Debug, Default, PartialEq)]
pub struct DbResponse {
    pub req_type: RequestType,
    pub payload: DbPayload,
    pub error: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct DbRequest {
    pub req_type: RequestType,
    pub payload: DbPayload,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub enum DbPayload {
    #[default]
    None,
    AffectedRows(u64),
    ItemParams(ItemParams),
    Item(StoreItem),
    Items(Vec<StoreItem>),
    ReceiptParams(JoinedReceiptParams),
    Receipt(StoreJoinRow),
    Receipts(Vec<StoreJoinRow>),
    UserParams(UserParams),
    User(StoreUser),
    Users(Vec<StoreUser>),
}

#[derive(Debug)]
pub enum DbPayloadBuilder {
    ItemParams(ItemParamsBuilder),
    UserParams(UserParamsBuilder),
    ReceiptParams(JoinParamsBuilder),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RequestType {
    #[default]
    None,
    GetAll,
    Get,
    Post,
    Update,
    Delete,
    Reset,
}

#[derive(Debug, Default, Clone)]
pub struct ParamOption<T>(Rc<RefCell<Option<T>>>)
where
    T: Default + Debug;

impl<T> ParamOption<T>
where
    T: Default + Debug + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn unwrap(&self) -> Option<T> {
        self.0.borrow().deref().clone()
    }
    pub fn map_value(&self, value: impl Into<T>) -> &Self {
        {
            *self.0.borrow_mut() = Some(value.into());
        }
        self
    }
}

#[derive(Debug, Default)]
pub struct UserParamsBuilder {
    pub u_id: ParamOption<i64>,
    pub name: ParamOption<String>,
}

#[derive(Debug, Default)]
pub struct ItemParamsBuilder {
    pub offset: Option<i64>,
    pub item_id: ParamOption<i64>,
    pub item_name: ParamOption<String>,
    pub item_price: ParamOption<f64>,
}

#[derive(Debug, Default)]
pub struct JoinParamsBuilder {
    pub offset: Option<i64>,
    pub users: Rc<RefCell<Vec<i64>>>,
    pub r_id: ParamOption<i64>,
    pub item_id: ParamOption<i64>,
    pub item_qty: ParamOption<i64>,
}
