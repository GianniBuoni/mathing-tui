use std::error::Error;

use sqlx::SqlitePool;

use crate::db::db_time::get_time;

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
