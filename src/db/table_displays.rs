use std::borrow::Cow;

use ratatui::widgets::Cell;

use super::{prelude::DbPayload, *};
use crate::table::TableDisplay;

impl TableDisplay for DbTable {
    fn ref_array(&self) -> Vec<Cell> {
        match self {
            DbTable::Item(i) => {
                let name = format!(" {} ", i.name);
                let price = format!(" {} ", i.price);
                vec![name.into(), price.into()]
            }
            DbTable::User(u) => {
                let name = format!(" {} ", u.name);
                vec![name.into()]
            }
            DbTable::Receipt(r) => {
                let name = format!(" {} ", r.item_name);
                let price = format!(" {} ", r.item_price);
                let qty = format!(" {} ", r.item_qty);
                let payees = r
                    .users
                    .iter()
                    .map(|user| Cow::Borrowed(user.name.as_str()))
                    .collect::<Vec<Cow<str>>>();
                let payees = payees.join(", ");
                let payees = format!(" {payees} ");
                vec![name.into(), price.into(), qty.into(), payees.into()]
            }
            _ => vec![],
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
