use super::*;
use super::{connection::get_db, requests::prelude::*};

use std::time::Duration;

use tokio::time::{Instant, sleep_until};

mod handle_req_errors;
mod handle_requests;
mod item_params;
mod join_params;
mod receipts_params;
mod receipts_users_params;
mod user_params;

#[sqlx::test]
async fn test_db_conn() {
    let conn = get_db().await;
    assert!(conn.is_ok());
}

pub const TEST_USERS: [&str; 3] = ["Thing", "Noodle", "Jon"];

pub const TEST_ITEMS: [(&str, f64, i64); 3] = [
    ("PB Pretzel", 4.99, 2),
    ("Slamin' Salmon", 9.49, 1),
    ("Chips and Dip", 5.55, 3),
];

pub fn intermediate_totals() -> Vec<HashMap<i64, Decimal>> {
    vec![
        HashMap::from([(3, dec!(9.98))]),
        HashMap::from([(2, dec!(9.49))]),
        HashMap::from([(2, dec!(8.32)), (3, dec!(8.32))]),
    ]
}

pub fn expected_totals() -> HashMap<i64, Decimal> {
    HashMap::from([(3, dec!(18.30)), (2, dec!(17.81))])
}

async fn init_users(conn: &SqlitePool) -> Result<Vec<StoreUser>> {
    let mut res = vec![];
    for name in TEST_USERS {
        let user = UserParams::new().user_name(name).post(conn).await?;
        res.push(user);
    }
    Ok(res)
}

async fn intit_items(conn: &SqlitePool) -> Result<Vec<(StoreItem, i64)>> {
    let mut res = vec![];
    for (name, price, qty) in TEST_ITEMS {
        let item = ItemParams::new()
            .item_name(name)
            .item_price(price)
            .post(conn)
            .await?;
        res.push((item, qty));
    }
    Ok(res)
}

async fn init_reciepts(conn: &SqlitePool) -> Result<Vec<StoreReceipt>> {
    anyhow::Ok(
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, qty)| {
            anyhow::Ok::<StoreReceipt>({
                let item = ItemParams::new()
                    .item_name(name)
                    .item_price(price)
                    .post(&conn)
                    .await?;

                let mut tx = conn.begin().await?;
                let receipts = ReceiptParams::new()
                    .item_id(item.id)
                    .item_qty(qty)
                    .post(&mut *tx)
                    .await?;
                tx.commit().await?;

                receipts
            })
        }))
        .await?,
    )
}
