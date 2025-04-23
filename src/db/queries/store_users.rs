use super::*;

pub async fn add_store_user(
    conn: &SqlitePool,
    name: &str,
) -> Result<StoreUser, Box<dyn Error>> {
    let now = get_time()?;

    let row = sqlx::query_as!(
        StoreUser,
        "
        INSERT INTO users (
            created_at, updated_at, name
        ) VALUES (
            ?1, ?2, ?3
        ) RETURNING *
        ",
        now,
        now,
        name
    )
    .fetch_one(conn)
    .await?;

    Ok(row)
}

pub async fn get_store_user_single(
    conn: &SqlitePool,
    id: i64,
) -> Result<StoreUser, Box<dyn Error>> {
    let row = sqlx::query_as!(StoreUser, "SELECT * FROM users WHERE id=?1", id)
        .fetch_one(conn)
        .await?;

    Ok(row)
}

pub async fn get_store_users(
    conn: &SqlitePool,
) -> Result<Vec<StoreUser>, Box<dyn Error>> {
    let rows = sqlx::query_as!(StoreUser, "SELECT * FROM users ORDER BY name")
        .fetch_all(conn)
        .await?;
    Ok(rows)
}

pub async fn delete_store_user(
    conn: &SqlitePool,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    sqlx::query!("DELETE FROM users WHERE id=?1", id)
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn update_store_user(
    conn: &SqlitePool,
    id: i64,
    name: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    if let Some(name) = name {
        let now = get_time()?;
        sqlx::query!(
            "UPDATE users SET updated_at=?1, name=?2 WHERE id=?3",
            now,
            name,
            id
        )
        .execute(conn)
        .await?;
    }

    Ok(())
}
