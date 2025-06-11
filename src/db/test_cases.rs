//! These are mock stucts for the rest of the crate
//! uses for testing purposes

use crate::prelude::*;

pub fn mock_items() -> TableData<StoreItem> {
    let test_1 = StoreItem {
        name: "Slamon".into(),
        price: 9.49,
        ..Default::default()
    };

    let test_2 = StoreItem {
        name: "Pretzels".into(),
        price: 5.59,
        ..Default::default()
    };

    let test_3 = StoreItem {
        name: "Blueberries".into(),
        price: 4.59,
        ..Default::default()
    };

    TableData::new_builder()
        .add_title("Grocery Items")
        .add_heading("Items")
        .add_heading("Price")
        .add_item(test_1)
        .add_item(test_2)
        .add_item(test_3)
        .build()
}

pub fn mock_receipts() -> TableData<StoreJoinRow> {
    let jon = StoreUser {
        name: "Jon".into(),
        ..Default::default()
    };

    let noodle = StoreUser {
        name: "Noodle".into(),
        ..Default::default()
    };

    let test_1 = StoreJoinRow {
        users: vec![jon.clone(), noodle.clone()],
        item_name: "Slamon".into(),
        item_price: 9.49,
        item_qty: 1,
        ..Default::default()
    };

    let test_2 = StoreJoinRow {
        users: vec![jon.clone()],
        item_name: "Blueberries".into(),
        item_price: 5.59,
        item_qty: 4,
        ..Default::default()
    };

    TableData::new_builder()
        .add_title("Receipt Items")
        .add_heading("Item Name")
        .add_heading("Item Price")
        .add_heading("Item Qty")
        .add_heading("Payees")
        .add_item(test_1)
        .add_item(test_2)
        .build()
}
