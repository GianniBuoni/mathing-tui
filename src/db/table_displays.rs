use std::borrow::Cow;

use ratatui::widgets::{Cell, Row};

use super::{prelude::DbPayload, *};
use crate::table::TableDisplay;

impl TableDisplay for DbTable {
    fn ref_array(&self) -> Row {
        match self {
            DbTable::Item(i) => {
                let name = format!(" {} ", i.name);
                let price = format!(" {:.2} ", i.price);
                Row::new([Cell::from(name), Cell::from(price)])
            }
            DbTable::User(u) => {
                let name = format!(" {} ", u.name);
                Row::new([Cell::from(name)])
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
