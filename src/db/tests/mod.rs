use super::*;
use super::{connection::get_db, requests::prelude::*};

use std::time::Duration;

use tokio::time::{Instant, sleep_until};

mod constants;
mod item_params;
mod join_params;
mod receipts_params;
mod receipts_users_params;
mod user_params;

use constants::*;

#[sqlx::test]
async fn test_db_conn() {
    let conn = get_db().await;
    assert!(conn.is_ok());
}

async fn init_users(conn: &SqlitePool) -> Result<Vec<StoreUser>> {
    anyhow::Ok(
        try_join_all(TEST_USERS.into_iter().map(async |name| {
            UserParams::new().user_name(name).post(conn).await
        }))
        .await?,
    )
}

async fn intit_items(conn: &SqlitePool) -> Result<Vec<(StoreItem, i64)>> {
    anyhow::Ok(
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, qty)| {
            anyhow::Ok::<(StoreItem, i64)>({
                let item = ItemParams::new()
                    .item_name(name)
                    .item_price(price)
                    .post(conn)
                    .await?;
                (item, qty)
            })
        }))
        .await?,
    )
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
