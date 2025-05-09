//! These are example structs that implement `[TableDisplay]`
//! for use within test cases

use std::borrow::Cow;

use crate::prelude::*;
use rust_decimal::{Decimal, dec};

pub fn test_rect() -> Rect {
    Rect::new(0, 0, 50, 8)
}

pub fn mock_items<'a>() -> TableData<'a, MockItems> {
    let item_headings = [" Items ", " Price "]
        .iter()
        .map(|string| Cow::Borrowed(*string))
        .collect();

    let mock_items = [
        MockItems::new("Slamon", dec!(9.49)),
        MockItems::new("Pretzels", dec!(5.59)),
        MockItems::new("Blueberries", dec!(4.59)),
    ];

    TableData::new("Grocery Items", item_headings, mock_items, 0)
}

pub fn mock_receipts<'a>() -> TableData<'a, MockReceipt> {
    let rec_headings =
        [" Item Name ", " Item Price ", " Item Qty ", " Payees "]
            .iter()
            .map(|string| Cow::Borrowed(*string))
            .collect();

    let mock_receipt = [
        MockReceipt::new("Slamon", "Jon, Noodle", dec!(9.49), 1),
        MockReceipt::new("Blueberries", "Jon", dec!(5.59), 4),
    ];

    TableData::new("Receipt Items", rec_headings, mock_receipt, 1)
}

pub fn test_app() -> App {
    let mut app = App::default();

    app.register_model(CurrentModel::Items, Box::new(mock_items()))
        .expect("Test app should be empty");
    app.register_model(CurrentModel::Receipt, Box::new(mock_receipts()))
        .expect("Test app shouldn't already have the Receipts key");

    app.init_view();
    app
}

#[derive(Debug, Default)]
pub struct MockItems {
    name: String,
    price: Decimal,
}

impl MockItems {
    fn new(name: &str, price: Decimal) -> Self {
        let name = name.into();
        Self { name, price }
    }
    fn name(&self) -> Cow<str> {
        Cow::Owned(format!(" {} ", self.name))
    }
    fn price(&self) -> Cow<str> {
        Cow::Owned(format!(" {} ", self.price))
    }
}

impl TableDisplay for MockItems {
    fn ref_array(&self) -> Vec<Cow<str>> {
        vec![self.name(), self.price()]
    }
}

#[derive(Debug, Default)]
pub struct MockReceipt {
    item_name: String,
    payees: String,
    item_price: Decimal,
    item_qty: i64,
}

impl MockReceipt {
    fn new(
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
        let item_name = Cow::Owned(format!(" {} ", self.item_name));
        let payees = Cow::Owned(format!(" {} ", self.payees.as_str()));
        let item_price: Cow<str> = Cow::Owned(format!(" {} ", self.item_price));
        let item_qty: Cow<str> = Cow::Owned(format!(" {} ", self.item_qty));

        vec![item_name, item_price, item_qty, payees]
    }
}
