use sqlx::{SqliteExecutor, query, query_as};

use super::*;

pub struct StoreItem {
    name: String,
    id: i64,
    price: f64,
}

impl StoreItem {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn price(&self) -> f64 {
        self.price
    }
}

pub async fn get_items(
    conn: impl SqliteExecutor<'_>,
) -> Result<Vec<StoreItem>, Box<dyn Error>> {
    let rows = query_as!(StoreItem, "SELECT * FROM items ORDER BY name")
        .fetch_all(conn)
        .await?;

    Ok(rows)
}

pub async fn add_items(
    db: impl SqliteExecutor<'_>,
    name: &str,
    price: f64,
) -> Result<StoreItem, Box<dyn Error>> {
    let new_item = query_as!(
        StoreItem,
        "INSERT INTO items (name, price) VALUES (?1, ?2) RETURNING *",
        name,
        price
    )
    .fetch_one(db)
    .await?;

    Ok(new_item)
}

pub async fn delete_items(
    db: impl SqliteExecutor<'_>,
    id: i64,
) -> Result<(), Box<dyn Error>> {
    query!("DELETE FROM items WHERE id=?1", id)
        .execute(db)
        .await?;
    Ok(())
}
