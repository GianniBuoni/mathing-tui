use sqlx::SqliteExecutor;

use super::*;

pub async fn get_store_receipts(
    conn: impl SqliteExecutor<'_>,
    offset: i64,
) -> Result<Vec<StoreReceipt>, Box<dyn Error>> {
    let res = sqlx::query_as!(
        StoreReceipt,
        "SELECT * FROM receipts ORDER BY updated_at LIMIT 20 OFFSET ?1",
        offset
    )
    .fetch_all(conn)
    .await?;
    Ok(res)
}

pub async fn get_store_receipt_single(
    conn: impl SqliteExecutor<'_>,
    id: i64,
) -> Result<StoreReceipt, Box<dyn Error>> {
    let res =
        sqlx::query_as!(StoreReceipt, "SELECT * FROM receipts WHERE id=?", id)
            .fetch_one(conn)
            .await?;

    Ok(res)
}

pub async fn add_store_receipt(
    conn: &SqlitePool,
    item_id: i64,
    qty: i64,
) -> Result<StoreReceipt, Box<dyn Error>> {
    let now = get_time()?;
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

pub async fn delete_store_receipt_single(
    conn: &SqlitePool,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!("DELETE FROM receipts WHERE id=?1", id)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn delete_store_receipts(
    conn: &SqlitePool,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!("DELETE FROM receipts").execute(conn).await?;
    Ok(())
}

pub async fn update_store_receipt(
    conn: &SqlitePool,
    id: i64,
    qty: Option<i64>,
) -> Result<(), Box<dyn Error>> {
    if let Some(qty) = qty {
        let now = get_time()?;

        sqlx::query!(
            "UPDATE receipts SET updated_at=?1, item_qty=?2 WHERE id=?3",
            now,
            qty,
            id
        )
        .execute(conn)
        .await?;
    }
    Ok(())
}
