//! These are mock stucts for the rest of the crate
//! uses for testing purposes

use crate::prelude::*;

impl StoreItem {
    pub fn mock() -> [Self; 3] {
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
}

impl StoreUser {
    pub fn mock() -> [Self; 2] {
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
}

impl StoreJoinRow {
    pub fn mock() -> [Self; 2] {
        let [jon, noodle] = StoreUser::mock();

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
}
