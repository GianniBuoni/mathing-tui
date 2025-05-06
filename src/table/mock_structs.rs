//! These are example structs that implement `[TableDisplay]`
//! for use within Tables

use super::*;

use rust_decimal::Decimal;

pub struct MockItems {
    name: String,
    price: Decimal,
}

impl MockItems {
    pub fn new(name: &str, price: Decimal) -> Self {
        let name = name.into();
        Self { name, price }
    }
    fn name(&self) -> Cow<str> {
        Cow::Borrowed(self.name.as_str())
    }
    fn price(&self) -> Cow<str> {
        Cow::Owned(format!("{}", self.price))
    }
}

impl TableDisplay for MockItems {
    fn ref_array(&self) -> Vec<Cow<str>> {
        vec![self.name(), self.price()]
    }
}

pub struct MockReceipt {
    item_name: String,
    payees: String,
    item_price: Decimal,
    item_qty: i64,
}

impl MockReceipt {
    pub fn new(
        item_name: &str,
        payees: &str,
        item_price: Decimal,
        item_qty: i64,
    ) -> Self {
        Self {
            item_name: item_name.into(),
            payees: payees.into(),
            item_price,
            item_qty,
        }
    }
}

impl TableDisplay for MockReceipt {
    fn ref_array(&self) -> Vec<Cow<str>> {
        let item_name = Cow::Borrowed(self.item_name.as_str());
        let payees = Cow::Borrowed(self.payees.as_str());
        let item_price: Cow<str> = Cow::Owned(format!("{}", self.item_price));
        let item_qty: Cow<str> = Cow::Owned(format!("{}", self.item_qty));

        vec![item_name, item_price, item_qty, payees]
    }
}
