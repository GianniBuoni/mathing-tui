use std::borrow::Cow;

use ratatui::widgets::{Cell, Row};

use super::*;

impl DbTable {
    /// Unwrapps a DbTable into an inner StoreItem.
    /// Errors out if the DbTable is not a StoreItem.
    pub fn try_get_item(&self) -> Result<&StoreItem, AppError> {
        match self {
            DbTable::Item(i) => Ok(i),
            DbTable::User(_) => {
                Err(AppError::Mapping(AppArm::Items, AppArm::Users))
            }
            DbTable::Receipt(_) => {
                Err(AppError::Mapping(AppArm::Items, AppArm::Receipts))
            }
        }
    }
    /// Unwrapps a DbTable into an inner StoreUser.
    /// Errors out if the DbTable is not a StoreUser.
    pub fn try_get_user(&self) -> Result<&StoreUser, AppError> {
        match self {
            DbTable::Item(_) => {
                Err(AppError::Mapping(AppArm::Users, AppArm::Items))
            }
            DbTable::User(u) => Ok(u),
            DbTable::Receipt(_) => {
                Err(AppError::Mapping(AppArm::Users, AppArm::Receipts))
            }
        }
    }
    /// Unwrapps a DbTable into an inner StoreJoinRow.
    /// Errors out if the DbTable is not a StoreJoinRow.
    pub fn try_get_receipt(&self) -> Result<&StoreJoinRow, AppError> {
        match self {
            DbTable::Item(_) => {
                Err(AppError::Mapping(AppArm::Receipts, AppArm::Items))
            }
            DbTable::User(_) => {
                Err(AppError::Mapping(AppArm::Receipts, AppArm::Users))
            }
            DbTable::Receipt(r) => Ok(r),
        }
    }
    /// Converts a reference to a DbTable into a Ratatui Row widget.
    pub fn into_row(&self) -> Row {
        match self {
            DbTable::Item(i) => {
                let name = format!(" {} ", i.name);
                let price = format!(" {:.2} ", i.price);
                Row::new([Cell::from(name), Cell::from(price)])
            }
            DbTable::User(u) => {
                let name = format!(" {} ", u.name);
                let totals =
                    StoreTotal::try_get_inner(u.id).unwrap_or_default();
                let totals = format!(" {totals:.2} ");
                Row::new([Cell::from(name), Cell::from(totals)])
            }
            DbTable::Receipt(r) => {
                let name = format!(" {} ", r.item_name);
                let price = format!(" {:.2} ", r.item_price);
                let qty = format!(" {} ", r.item_qty);
                let payees = r
                    .users
                    .iter()
                    .map(|user| Cow::Borrowed(user.name.as_str()))
                    .collect::<Vec<Cow<str>>>();
                let payees = payees.join(", ");
                let payees = format!(" {payees} ");
                let cells: [Cell; 4] =
                    [name.into(), price.into(), qty.into(), payees.into()];
                Row::new(cells)
            }
        }
    }
}
impl From<DbPayload> for Vec<DbTable> {
    fn from(value: DbPayload) -> Self {
        match value {
            DbPayload::Item(i) => vec![DbTable::Item(i)],
            DbPayload::Receipt(r) => vec![DbTable::Receipt(r)],
            DbPayload::User(u) => vec![DbTable::User(u)],
            DbPayload::Items(i) => i.into_iter().map(DbTable::Item).collect(),
            DbPayload::Receipts(r) => {
                r.into_iter().map(DbTable::Receipt).collect()
            }
            DbPayload::Users(u) => u.into_iter().map(DbTable::User).collect(),
            _ => vec![],
        }
    }
}
