use super::*;

impl DbConn {
    pub async fn try_init(db_dir: PathBuf) -> Result<Self> {
        let db_str = db_dir.to_str().ok_or(Error::msg(
            "Couldn't parse: \"{db_dir:?}\" as a connection string.",
        ))?;
        let pool = SqlitePool::connect(db_str).await?;

        sqlx::query!(
            "
CREATE TABLE IF NOT EXISTS items (
  id INTEGER PRIMARY KEY NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  name TEXT NOT NULL,
  price FLOAT NOT NULL
);
CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  name TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS receipts(
  id INTEGER PRIMARY KEY NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  item_id INTEGER NOT NULL,
  item_qty INTEGER NOT NULL,
  FOREIGN KEY(item_id) REFERENCES items(id)
    ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS receipts_users (
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  receipt_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  FOREIGN KEY(receipt_id) REFERENCES receipts(id)
    ON DELETE CASCADE,
  FOREIGN KEY(user_id) REFERENCES users(id)
    ON DELETE CASCADE,
  UNIQUE(receipt_id, user_id)
);
            "
        )
        .execute(&pool)
        .await?;

        Ok(Self(pool))
    }
}
