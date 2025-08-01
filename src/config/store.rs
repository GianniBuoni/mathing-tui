use super::*;

impl DbConn {
    pub(super) async fn try_init(db_dir: &Path) -> Result<Self> {
        let db_str = db_dir.to_str().ok_or_else(|| {
            let message = format!(
                "Couldn't parse \"{db_dir:?}\" as a connection string."
            );
            AppError::config(message)
        })?;
        let pool = SqlitePool::connect(db_str).await?;

        // run migrations if tabless don't exist
        sqlx::query_file!("sql/init.sql").execute(&pool).await?;

        Ok(Self(pool))
    }

    pub fn try_get() -> Result<&'static SqlitePool> {
        Ok(&CONFIG.get().ok_or(AppError::ConfigInit)?.store.0)
    }
}
