//! These are mock stucts for the rest of the crate
//! uses for testing purposes

use crate::prelude::*;

pub fn mock_items() -> [StoreItem; 3] {
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

    [test_1, test_2, test_3]
}

pub fn mock_users() -> [StoreUser; 2] {
    let jon = StoreUser {
        name: "Jon".into(),
        ..Default::default()
    };

    let noodle = StoreUser {
        name: "Noodle".into(),
        ..Default::default()
    };

    [jon, noodle]
}

pub fn mock_receipts() -> [StoreJoinRow; 2] {
    let [jon, noodle] = mock_users();

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

    [test_1, test_2]
}

pub fn mock_items_table() -> TableData<StoreItem> {
    let mut table = TableData::<StoreItem>::new_builder()
        .add_title("Grocery Items")
        .add_heading("Items")
        .add_heading("Price")
        .build();

    mock_items()
        .into_iter()
        .for_each(|item| table.add_item(item));

    table
}

pub fn mock_receipts_table() -> TableData<StoreJoinRow> {
    let mut table = TableData::<StoreJoinRow>::new_builder()
        .add_title("Receipt Items")
        .add_heading("Item Name")
        .add_heading("Item Price")
        .add_heading("Item Qty")
        .add_heading("Payees")
        .build();

    mock_receipts()
        .into_iter()
        .for_each(|item| table.add_item(item));

    table
}
