use std::{collections::HashMap, default};

use super::*;

pub async fn init_test(conn: &SqlitePool) -> Result<(), Box<dyn Error>> {
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

    let mut pairs = HashMap::new();
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

    Ok(())
}
