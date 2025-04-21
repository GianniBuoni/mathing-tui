use std::{env, error::Error};

use sqlx::SqlitePool;
use tokio::sync::OnceCell;

mod store_items;

pub mod prelude {
    pub use super::get_db;
    pub use super::store_items::prelude::*;
}

static DB: OnceCell<SqlitePool> = OnceCell::const_new();

async fn db() -> Result<SqlitePool, Box<dyn Error>> {
    let db_string = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_string).await?;

    Ok(pool)
}

pub async fn get_db() -> Result<&'static SqlitePool, Box<dyn Error>> {
    match DB.get_or_try_init(db).await {
        Ok(static_pool) => Ok(static_pool),
        Err(e) => {
            let msg = format!("DB connection error: {e}");
            Err(msg.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_db_conn() {
        let conn = get_db().await;
        assert!(conn.is_ok());
    }
}
