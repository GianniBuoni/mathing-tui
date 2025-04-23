use super::*;

pub async fn get_store_items(
    conn: &SqlitePool,
) -> Result<Vec<StoreItem>, Box<dyn Error>> {
    let rows = sqlx::query_as!(StoreItem, "SELECT * FROM items ORDER BY name")
        .fetch_all(conn)
        .await?;

    Ok(rows)
}

pub async fn get_store_item_single(
    conn: &SqlitePool,
    id: i64,
) -> Result<StoreItem, Box<dyn Error>> {
    let item =
        sqlx::query_as!(StoreItem, "SELECT * FROM items WHERE id=?1", id)
            .fetch_one(conn)
            .await?;

    Ok(item)
}

pub async fn add_store_item(
    conn: &SqlitePool,
    name: &str,
    price: f64,
) -> Result<StoreItem, Box<dyn Error>> {
    let now = get_time()?;

    let new_item = sqlx::query_as!(
        StoreItem,
        "
        INSERT INTO items (
            name, price, created_at, updated_at
        ) VALUES (
            ?1, ?2, ?3, ?4
        ) RETURNING *
        ",
        name,
        price,
        now,
        now,
    )
    .fetch_one(conn)
    .await?;

    Ok(new_item)
}

pub async fn delete_store_item(
    conn: &SqlitePool,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!("DELETE FROM items WHERE id=?1", id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn update_store_item(
    conn: &SqlitePool,
    id: i64,
    name: Option<&str>,
    price: Option<f64>,
) -> Result<(), Box<dyn Error>> {
    // early return if theres's nothing to update
    if name.is_none() && price.is_none() {
        return Ok(());
    }

    // begin transaction
    let mut tx = conn.begin().await?;
    let now = get_time()?;

    sqlx::query!("UPDATE items SET updated_at=?1 WHERE id=?2", now, id)
        .execute(&mut *tx)
        .await?;

    if let Some(name) = name {
        sqlx::query!("UPDATE items SET name=?1 WHERE id=?2", name, id)
            .execute(&mut *tx)
            .await?;
    };

    if let Some(price) = price {
        sqlx::query!("UPDATE items SET price=?1 WHERE id=?2", price, id)
            .execute(&mut *tx)
            .await?;
    };

    // commit changes
    tx.commit().await?;
    Ok(())
}
