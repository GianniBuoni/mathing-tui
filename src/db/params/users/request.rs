use super::*;

impl Request for UserParams {
    type Output = StoreUser;
    type Outputs = Vec<StoreUser>;

    fn get_app_arm(&self) -> AppArm {
        AppArm::Users
    }

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.u_id
            .ok_or(RequestError::missing_param(req_type, "user", "id"))
    }

    async fn get_all(&self, conn: &SqlitePool) -> Result<Self::Outputs> {
        Ok(
            sqlx::query_as!(Self::Output, "SELECT * FROM users ORDER BY name")
                .fetch_all(conn)
                .await?,
        )
    }

    async fn get(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Get)?;
        Ok(
            sqlx::query_as!(StoreUser, "SELECT * FROM users WHERE id=?1", id)
                .fetch_one(conn)
                .await
                .map_err(|_| RequestError::not_found(id, "users"))?,
        )
    }

    async fn post(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let now = AppConfig::try_get_time()?;

        let name = self.name.clone().ok_or(RequestError::missing_param(
            RequestType::Post,
            "user",
            "name",
        ))?;

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

    async fn delete(&self, conn: &SqlitePool) -> Result<u64> {
        let id = self.check_id(RequestType::Delete)?;
        let mut tx = conn.begin().await?;

        let res = sqlx::query!("DELETE FROM users WHERE id=?1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(res.rows_affected())
    }

    async fn update(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;

        let id = self.check_id(RequestType::Update)?;
        let now = AppConfig::try_get_time()?;
        let name = self.name.clone().ok_or(RequestError::missing_param(
            RequestType::Update,
            "user",
            "name",
        ))?;

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

    async fn count(&self, conn: &SqlitePool) -> Result<i64> {
        Ok(
            sqlx::query_as!(StoreCount, "SELECT COUNT(*) AS rows FROM users")
                .fetch_one(conn)
                .await
                .unwrap_or_default()
                .rows,
        )
    }
}
