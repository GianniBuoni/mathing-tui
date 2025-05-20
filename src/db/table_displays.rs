use std::borrow::Cow;

use super::*;
use crate::table::TableDisplay;

impl TableDisplay for StoreItem {
    fn ref_array(&self) -> Vec<std::borrow::Cow<str>> {
        let name = Cow::Borrowed::<str>(self.name.as_ref());
        let price = Cow::Owned(format!("{}", self.price));
        vec![name, price]
    }
}

impl TableDisplay for StoreJoinRow {
    fn ref_array(&self) -> Vec<Cow<str>> {
        let item_name = Cow::Borrowed(self.item_name.as_ref());
        let item_price = Cow::Owned(format!("{}", self.item_price));
        let item_qty = Cow::Owned(format!("{}", self.item_qty));
        let payees = self
            .users
            .iter()
            .map(|user| format!("{}", user.name))
            .collect::<Vec<String>>()
            .join(",");
        let payees = Cow::Owned(payees);

        vec![item_name, item_price, item_qty, payees]
    }
}
