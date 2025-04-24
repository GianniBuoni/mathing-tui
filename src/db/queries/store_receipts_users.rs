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
    let mut rows = vec![];

    let receipts = get_store_receipts(conn, offset).await?;
    for receipt in receipts {
        let join_row = get_receipts_users(conn, receipt.id).await?;
        rows.push(join_row);
    }

    Ok(rows)
}

pub async fn get_receipts_users(
    conn: &SqlitePool,
    r_id: i64,
) -> Result<StoreJoinRow, Box<dyn Error>> {
    // get receipt
    let StoreReceipt {
        id: receipt_id,
        item_id,
        item_qty,
        ..
    } = get_store_receipt_single(conn, r_id).await?;

    // get item name
    let StoreItem {
        name: item_name,
        price: item_price,
        ..
    } = get_store_item_single(conn, item_id).await?;

    // get join table
    let join_rows = sqlx::query_as!(
        StoreReceiptsUsers,
        "
        SELECT * FROM receipts_users WHERE receipt_id=?1
        ",
        r_id
    )
    .fetch_all(conn)
    .await?;

    // compose user ids
    let mut users = vec![];
    for join_row in join_rows {
        let user = get_store_user_single(conn, join_row.user_id).await?;
        users.push(user);
    }
    let payee_count = users.len();

    // return receipt joined
    Ok(StoreJoinRow {
        users,
        item_name,
        receipt_id,
        item_id,
        item_price,
        item_qty,
        user_count: payee_count,
    })
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
