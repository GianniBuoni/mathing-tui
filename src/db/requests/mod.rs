use sqlx::SqliteExecutor;

use super::*;

pub mod prelude {
    pub use super::errors::RequestError;
    pub use super::handle_requests::handle_requests;
    pub use super::{
        DbPayload, DbPayloadBuilder, DbRequest, DbResponse, ItemParamsBuilder,
        JoinParamsBuilder, Request, RequestType, UserParamsBuilder,
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

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default)]
pub enum DbPayloadBuilder {
    #[default]
    None,
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

#[derive(Debug, Default)]
pub struct ParamOption<T>(Option<Rc<RefCell<T>>>)
where
    T: Default + Debug;

impl<T> ParamOption<T>
where
    T: Default + Debug + Clone,
{
    pub fn new(value: T) -> Self {
        Self(Some(Rc::new(RefCell::new(value))))
    }
    pub fn unwrap(self) -> Option<T> {
        let inner_value = self.0.as_ref()?;
        let inner_value = inner_value.borrow();

        Some(inner_value.to_owned())
    }
}

#[derive(Debug, Default)]
pub struct UserParamsBuilder {
    u_id: ParamOption<i64>,
    name: ParamOption<String>,
}

#[derive(Debug, Default)]
pub struct ItemParamsBuilder {
    offset: Option<i64>,
    item_id: ParamOption<i64>,
    item_name: ParamOption<String>,
    item_price: ParamOption<f64>,
}

#[derive(Debug, Default)]
pub struct JoinParamsBuilder {
    offset: Option<i64>,
    users: Rc<RefCell<Vec<i64>>>,
    r_id: ParamOption<i64>,
    item_id: ParamOption<i64>,
    item_qty: ParamOption<i64>,
}
