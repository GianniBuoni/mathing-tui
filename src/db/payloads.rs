use super::*;

pub mod prelude {
    pub use super::{DbPayload, DbPayloadBuilder};
}

use super::params::{
    items::ItemParamsBuilder, join_row::JoinParamsBuilder,
    users::UserParamsBuilder,
};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum DbPayload {
    #[default]
    None,
    AffectedRows(u64),
    Count(AppArm, i64),
    ItemParams(ItemParams),
    Item(StoreItem),
    Items(Vec<StoreItem>),
    ReceiptParams(JoinedReceiptParams),
    Receipt(StoreJoinRow),
    Receipts(Vec<StoreJoinRow>),
    StoreTotal,
    UserParams(UserParams),
    User(StoreUser),
    Users(Vec<StoreUser>),
}

#[derive(Debug)]
pub enum DbPayloadBuilder {
    ItemParams(ItemParamsBuilder),
    UserParams(UserParamsBuilder),
    ReceiptParams(JoinParamsBuilder),
    StoreTotal,
}

impl Display for DbPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::AffectedRows(_) => write!(f, "AffectedRows"),
            Self::Count(_, _) => write!(f, "Count"),
            Self::ItemParams(_) => write!(f, "ItemParams"),
            Self::Item(_) => write!(f, "Item"),
            Self::Items(_) => write!(f, "Items"),
            Self::ReceiptParams(_) => write!(f, "ReceiptParams"),
            Self::Receipt(_) => write!(f, "Receipt"),
            Self::Receipts(_) => write!(f, "Receipts"),
            Self::StoreTotal => write!(f, "Totals"),
            Self::UserParams(_) => write!(f, "UserParams"),
            Self::User(_) => write!(f, "User"),
            Self::Users(_) => write!(f, "Users"),
        }
    }
}

impl DbPayloadBuilder {
    pub fn build(&self) -> DbPayload {
        match self {
            Self::ItemParams(i) => DbPayload::ItemParams(i.build()),
            Self::UserParams(u) => DbPayload::UserParams(u.build()),
            Self::ReceiptParams(r) => DbPayload::ReceiptParams(r.build()),
            Self::StoreTotal => DbPayload::StoreTotal,
        }
    }
}

// From implementations
impl From<u64> for DbPayload {
    fn from(value: u64) -> Self {
        Self::AffectedRows(value)
    }
}
impl From<StoreItem> for DbPayload {
    fn from(value: StoreItem) -> Self {
        Self::Item(value)
    }
}
impl From<Vec<StoreItem>> for DbPayload {
    fn from(value: Vec<StoreItem>) -> Self {
        Self::Items(value)
    }
}
impl From<StoreUser> for DbPayload {
    fn from(value: StoreUser) -> Self {
        Self::User(value)
    }
}
impl From<Vec<StoreUser>> for DbPayload {
    fn from(value: Vec<StoreUser>) -> Self {
        Self::Users(value)
    }
}
impl From<StoreJoinRow> for DbPayload {
    fn from(value: StoreJoinRow) -> Self {
        Self::Receipt(value)
    }
}
impl From<Vec<StoreJoinRow>> for DbPayload {
    fn from(value: Vec<StoreJoinRow>) -> Self {
        Self::Receipts(value)
    }
}
