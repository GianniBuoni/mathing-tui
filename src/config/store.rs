use super::*;

impl DbConn {
    pub(super) async fn try_init(db_dir: PathBuf) -> Result<Self> {
        let db_str = db_dir.to_str().ok_or(Error::msg(
            "Couldn't parse: \"{db_dir:?}\" as a connection string.",
        ))?;
        let pool = SqlitePool::connect(db_str).await?;

        // run migrations if tabless don't exist
        sqlx::query_file!("sql/init.sql").execute(&pool).await?;
        Ok(Self(pool))
    }

    pub fn try_get() -> Result<&'static SqlitePool> {
        Ok(&CONFIG
            .get()
            .ok_or(AppError::config("Config hasn't been initialized yet"))?
            .store
            .0)
    }
}
