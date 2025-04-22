use std::{
    error::Error,
    time::{self, UNIX_EPOCH},
};

use super::*;

use sqlx::SqlitePool;

pub async fn get_store_receipts(
    conn: &SqlitePool,
) -> Result<Vec<StoreReceipt>, Box<dyn Error>> {
    let res = sqlx::query_as!(StoreReceipt, "SELECT * FROM receipts")
        .fetch_all(conn)
        .await?;
    Ok(res)
}

pub async fn add_store_receipt(
    conn: &SqlitePool,
    item_id: i64,
    qty: i64,
) -> Result<StoreReceipt, Box<dyn Error>> {
    let now = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs() as i64;
    let res = sqlx::query_as!(
        StoreReceipt,
        "
        INSERT INTO receipts (
            created_at, updated_at, item_id, item_qty
        ) VALUES (
            ?1, ?2, ?3, ?4
        ) RETURNING *
        ",
        now,
        now,
        item_id,
        qty,
    )
    .fetch_one(conn)
    .await?;

    Ok(res)
}

pub async fn delete_store_receipt(
    conn: &SqlitePool,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!("DELETE FROM receipts WHERE id=?1", id)
        .execute(conn)
        .await?;

    Ok(())
}
