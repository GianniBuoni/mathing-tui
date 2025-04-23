use std::{collections::HashMap, default};

use super::*;

const TEST_ITEMS: [(&str, f64, i64); 3] = [
    ("PB Pretzel", 4.99, 2),
    ("Slamin' Salmon", 9.49, 1),
    ("Chips and Dip", 5.55, 3),
];

const TEST_USERS: [&str; 3] = ["Thing", "Noodle", "Jon"];

// returns kv of receipts -> Vec<users>
async fn init_test(
    conn: &SqlitePool,
) -> Result<HashMap<i64, Vec<i64>>, Box<dyn Error>> {
    let mut pairs = HashMap::new();

    let mut r_ids = vec![];

    for (name, price, qty) in TEST_ITEMS {
        let item_row = add_store_item(conn, name, price).await?;
        let receipt_row = add_store_receipt(conn, item_row.id, qty).await?;
        r_ids.push(receipt_row.id);
    }

    let mut u_ids = vec![];

    for name in TEST_USERS {
        let user_row = add_store_user(conn, name).await?;
        u_ids.push(user_row.id);
    }

    let err = "Wrong id/user match check how you add users into db.";
    let jon = get_store_user_single(conn, 3).await?;
    if jon.name != TEST_USERS[2] {
        return Err(err.into());
    }
    let noodle = get_store_user_single(conn, 2).await?;
    if noodle.name != TEST_USERS[1] {
        return Err(err.into());
    }

    let mut r_ids = r_ids.iter();
    // Add "Jon" to PB Pretzel, 2
    if let Some(id) = r_ids.next() {
        let v = vec![jon.id];
        pairs.insert(*id, v);
    }

    // Add "Noodle" to Slamin' Salmon, 1
    if let Some(id) = r_ids.next() {
        let v = vec![noodle.id];
        pairs.insert(*id, v);
    }

    // Add "Noodle" and "Jon"  to Chips and Dip, 3
    if let Some(id) = r_ids.next() {
        let v = vec![jon.id, noodle.id];
        pairs.insert(*id, v);
    }

    for (r_id, u_ids) in pairs.iter() {
        for u_id in u_ids {
            add_store_receipts_users(&conn, *r_id, *u_id).await?;
        }
    }

    Ok(pairs)
}

#[sqlx::test]
async fn test_add_receipts_users(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_get_receipts_joined(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let want = [
        (
            StoreJoinRow {
                receipt_id: 1,
                item_id: 1,
                payee: "Jon".into(),
                item_name: "PB Pretzel".into(),
                item_qty: 2,
                item_price: 4.99,
                payee_count: 1,
            },
            "Test adding single user to receipt_id.",
        ),
        (
            StoreJoinRow {
                receipt_id: 2,
                item_id: 2,
                payee: "Noodle".into(),
                item_name: "Slamin' Salmon".into(),
                item_qty: 1,
                item_price: 9.49,
                payee_count: 1,
            },
            "Test adding single user to receipt_id.",
        ),
        (
            StoreJoinRow {
                receipt_id: 3,
                item_id: 3,
                payee: "Noodle,Jon".into(),
                item_name: "Chips and Dip".into(),
                item_qty: 3,
                item_price: 5.55,
                payee_count: 2,
            },
            "Test add multiple users to receipt_id",
        ),
    ];

    let got = get_store_recipts_joined(&conn, 0).await?;

    for ((want, desc), got) in want.iter().zip(got) {
        assert_eq!(*want, got, "{desc}");
    }

    Ok(())
}
