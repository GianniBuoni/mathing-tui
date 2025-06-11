use ratatui::widgets::Cell;

use super::*;
use crate::table::TableDisplay;

impl TableDisplay for StoreItem {
    fn ref_array(&self) -> Vec<Cell> {
        let name = format!(" {} ", self.name);
        let price = format!(" {} ", self.price);
        vec![name.into(), price.into()]
    }
}

impl TableDisplay for StoreJoinRow {
    fn ref_array(&self) -> Vec<Cell> {
        let item_name = format!(" {} ", self.item_name);
        let item_price = format!(" {} ", self.item_price);
        let item_qty = format!(" {} ", self.item_qty);
        let payees = self
            .users
            .iter()
            .map(|user| user.name.to_owned())
            .collect::<Vec<String>>()
            .join(", ");
        let payees = format!(" {payees} ");

        vec![
            item_name.into(),
            item_price.into(),
            item_qty.into(),
            payees.into(),
        ]
    }
}
