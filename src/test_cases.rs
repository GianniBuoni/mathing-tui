//! These are example structs that implement `[TableDisplay]`
//! for use within test cases

use std::borrow::Cow;

use crate::prelude::*;
use rust_decimal::{Decimal, dec};

pub fn test_rect() -> Rect {
    Rect::new(0, 0, 50, 8)
}

pub fn mock_items<'a>() -> TableData<'a, MockItems> {
    TableData::new_builder()
        .add_index(0)
        .add_title("Grocery Items")
        .add_heading("Items")
        .add_heading("Price")
        .add_item(MockItems::TEST_1)
        .add_item(MockItems::TEST_2)
        .add_item(MockItems::TEST_3)
        .build()
}

pub fn mock_receipts<'a>() -> TableData<'a, MockReceipt> {
    TableData::new_builder()
        .add_index(1)
        .add_title("Receipt Items")
        .add_heading("Item Name")
        .add_heading("Item Price")
        .add_heading("Item Qty")
        .add_heading("Payees")
        .add_item(MockReceipt::TEST_1)
        .add_item(MockReceipt::TEST_2)
        .build()
}

pub fn test_home() -> Home {
    Home::new_builder()
        .add_component(Box::new(mock_items()))
        .add_component(Box::new(mock_receipts()))
        .build()
}

pub fn test_app() -> App {
    App::new(test_home())
}

#[derive(Debug, Default)]
pub struct MockItems {
    name: &'static str,
    price: Decimal,
}

impl<'a> MockItems {
    const TEST_1: Self = Self {
        name: " Slamon ",
        price: dec!(9.49),
    };
    const TEST_2: Self = Self {
        name: " Pretzels ",
        price: dec!(5.59),
    };
    const TEST_3: Self = Self {
        name: " Blueberries ",
        price: dec!(4.59),
    };
    fn price(&self) -> Cow<'a, str> {
        Cow::Owned(format!(" {} ", self.price))
    }
}

impl TableDisplay for MockItems {
    fn ref_array(&self) -> Vec<Cow<str>> {
        let name = Cow::Borrowed(self.name);
        vec![name, self.price()]
    }
}

#[derive(Debug, Default)]
pub struct MockReceipt {
    item_name: &'static str,
    payees: &'static str,
    item_price: Decimal,
    item_qty: i64,
}

impl<'a> MockReceipt {
    const TEST_1: Self = Self {
        item_name: " Slamon ",
        payees: " Jon, Noodle ",
        item_price: dec!(9.49),
        item_qty: 1,
    };
    const TEST_2: Self = Self {
        item_name: " Blueberries ",
        payees: " Jon ",
        item_price: dec!(5.59),
        item_qty: 4,
    };
    fn price(&self) -> Cow<'a, str> {
        Cow::Owned(format!(" {} ", self.item_price))
    }
    fn qty(&self) -> Cow<'a, str> {
        Cow::Owned(format!(" {} ", self.item_qty))
    }
}

impl TableDisplay for MockReceipt {
    fn ref_array(&self) -> Vec<std::borrow::Cow<str>> {
        let item_name = Cow::Borrowed(self.item_name);
        let payees = Cow::Borrowed(self.payees);
        vec![item_name, self.price(), self.qty(), payees]
    }
}
