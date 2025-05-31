use super::{errors::RequestError, *};

impl<'db> UserParams<'db> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn user_id(mut self, id: i64) -> Self {
        self.u_id = Some(id);
        self
    }
    pub fn user_name(mut self, name: impl Into<Cow<'db, str>>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl<'e> Request<'e> for UserParams<'_> {
    type Output = StoreUser;
    type Connection = &'e SqlitePool;

    fn check_id(&self) -> Result<i64> {
        Ok(self.u_id.ok_or(RequestError::missing_param("id"))?)
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let id = self.check_id()?;
        Ok(
            sqlx::query_as!(StoreUser, "SELECT * FROM users WHERE id=?1", id)
                .fetch_one(conn)
                .await
                .map_err(|_| RequestError::not_found(id, "users"))?,
        )
    }

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let now = get_time()?;

        let name = self
            .name
            .clone()
            .ok_or(RequestError::missing_param("name"))?;

        let user = sqlx::query_as!(
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
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
        let id = self.check_id()?;
        let mut tx = conn.begin().await?;

        let res = sqlx::query!("DELETE FROM users WHERE id=?1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(res.rows_affected())
    }

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;

        let id = self.check_id()?;
        let now = get_time()?;
        let name = self
            .name
            .clone()
            .ok_or(RequestError::missing_param("name"))?;

        sqlx::query!(
            "UPDATE users SET updated_at=?1, name=?2 WHERE id=?3",
            now,
            name,
            id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        self.get(conn).await
    }
}
