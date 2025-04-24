use super::*;

pub async fn add_store_receipts_users(
    conn: &SqlitePool,
    r_id: i64,
    u_id: i64,
) -> Result<(), Box<dyn Error>> {
    let now = get_time()?;
    sqlx::query!(
        "
    INSERT INTO receipts_users (
    created_at, updated_at, receipt_id, user_id
    ) VALUES (
        ?1, ?2, ?3, ?4
    )",
        now,
        now,
        r_id,
        u_id
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn get_store_joined_rows(
    conn: &SqlitePool,
    offset: i64,
) -> Result<Vec<StoreJoinRow>, Box<dyn Error>> {
    let rows = sqlx::query_file_as!(
        StoreJoinRow,
        "sql/get_receipts_joined.sql",
        offset
    )
    .fetch_all(conn)
    .await?;

    Ok(rows)
}

pub async fn delete_store_receipts_users(
    conn: &SqlitePool,
    r_id: i64,
    u_id: i64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!(
        "DELETE FROM receipts_users WHERE receipt_id=?1 and user_id=?2",
        r_id,
        u_id,
    )
    .execute(conn)
    .await?;

    Ok(())
}
