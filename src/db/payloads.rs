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
    Count(i64),
    ItemParams(ItemParams),
    Item(StoreItem),
    Items(Vec<StoreItem>),
    ReceiptParams(JoinedReceiptParams),
    Receipt(StoreJoinRow),
    Receipts(Vec<StoreJoinRow>),
    StoreTotal(TotalsParams),
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

impl Display for DbPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::AffectedRows(_) => write!(f, "AffectedRows"),
            Self::Count(_) => write!(f, "Count"),
            Self::ItemParams(_) => write!(f, "ItemParams"),
            Self::Item(_) => write!(f, "Item"),
            Self::Items(_) => write!(f, "Items"),
            Self::ReceiptParams(_) => write!(f, "ReceiptParams"),
            Self::Receipt(_) => write!(f, "Receipt"),
            Self::Receipts(_) => write!(f, "Receipts"),
            Self::StoreTotal(_) => write!(f, "Totals"),
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
        }
    }
}
