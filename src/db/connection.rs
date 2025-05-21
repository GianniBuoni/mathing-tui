use std::env;

use tokio::sync::OnceCell;

use super::*;

static DB: OnceCell<SqlitePool> = OnceCell::const_new();

async fn db() -> Result<SqlitePool> {
    let db_string = env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&db_string).await?;

    Ok(pool)
}

pub async fn get_db() -> Result<&'static SqlitePool> {
    DB.get_or_try_init(db).await
}
