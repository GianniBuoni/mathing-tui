use super::*;

pub const TEST_USERS: [&str; 3] = ["Thing", "Noodle", "Jon"];

pub const TEST_ITEMS: [(&str, f64, i64); 3] = [
    ("PB Pretzel", 4.99, 2),
    ("Slamin' Salmon", 9.49, 1),
    ("Chips and Dip", 5.55, 3),
];

pub fn expected_sql_rows() -> [StoreJoinRaw; 3] {
    [
        StoreJoinRaw {
            item_name: "PB Pretzel".into(),
            user_ids: "3".into(),
            receipt_id: 1,
            item_id: 1,
            item_price: 4.99,
            item_qty: 2,
            user_count: 1,
        },
        StoreJoinRaw {
            item_name: "Slamin' Salmon".into(),
            user_ids: "2".into(),
            receipt_id: 2,
            item_id: 2,
            item_price: 9.49,
            item_qty: 1,
            user_count: 1,
        },
        StoreJoinRaw {
            item_name: "Chips and Dip".into(),
            user_ids: "2,3".into(),
            receipt_id: 3,
            item_id: 3,
            item_price: 5.55,
            item_qty: 3,
            user_count: 2,
        },
    ]
}

pub fn expected_joined_rows(test_users: &Vec<StoreUser>) -> [StoreJoinRow; 3] {
    [
        StoreJoinRow {
            receipt_id: 1,
            item_id: 1,
            users: vec![test_users[0].clone()],
            item_name: "PB Pretzel".into(),
            item_qty: 2,
            item_price: 4.99,
            user_count: 1,
        },
        StoreJoinRow {
            receipt_id: 2,
            item_id: 2,
            users: vec![test_users[1].clone()],
            item_name: "Slamin' Salmon".into(),
            item_qty: 1,
            item_price: 9.49,
            user_count: 1,
        },
        StoreJoinRow {
            receipt_id: 3,
            item_id: 3,
            users: vec![test_users[1].clone(), test_users[0].clone()],
            item_name: "Chips and Dip".into(),
            item_qty: 3,
            item_price: 5.55,
            user_count: 2,
        },
    ]
}

pub fn expected_totals() -> [StoreJoinTotal; 4] {
    [
        StoreJoinTotal {
            receipt_id: 1,
            user_id: 3,
            total: 9.98,
        },
        StoreJoinTotal {
            receipt_id: 2,
            user_id: 2,
            total: 9.49,
        },
        StoreJoinTotal {
            receipt_id: 3,
            user_id: 3,
            total: 8.325,
        },
        StoreJoinTotal {
            receipt_id: 3,
            user_id: 2,
            total: 8.325,
        },
    ]
}
