use super::*;

impl Transaction for ReceiptsUsersParams {
    type Output = Vec<StoreReceiptsUsers>;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.r_id.ok_or(RequestError::missing_param(
            req_type,
            "receipts users",
            "receipt id",
        ))
    }

    async fn get(&self, conn: &mut SqliteConnection) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Get)?;

        let res = sqlx::query_as!(
            StoreReceiptsUsers,
            "
        SELECT * FROM receipts_users WHERE receipt_id=?1
                ",
            id,
        )
        .fetch_all(conn)
        .await?;

        if res.is_empty() {
            Err(RequestError::not_found(id, "receipts_users").into())
        } else {
            Ok(res)
        }
    }

    async fn post(&self, conn: &mut SqliteConnection) -> Result<Self::Output> {
        let r_id = self.check_id(RequestType::Post)?;

        let u_id = self.u_id.ok_or(RequestError::missing_param(
            RequestType::Post,
            "receipts users",
            "user id",
        ))?;
        let now = AppConfig::try_get_time()?;

        Ok(sqlx::query_as!(
            StoreReceiptsUsers,
            "
            INSERT INTO receipts_users (
                receipt_id, user_id, created_at, updated_at
            )
            VALUES (?1, ?2, ?3, ?4) RETURNING *
            ",
            r_id,
            u_id,
            now,
            now,
        )
        .fetch_all(conn)
        .await?)
    }

    async fn delete(&self, conn: &mut SqliteConnection) -> Result<u64> {
        let id = self.check_id(RequestType::Delete)?;
        let u_id = self.u_id.ok_or(RequestError::missing_param(
            RequestType::Delete,
            "receipts users",
            "user id",
        ))?;

        Ok(sqlx::query!(
            "
            DELETE FROM receipts_users WHERE receipt_id=?1 AND user_id=?2
            ",
            id,
            u_id,
        )
        .execute(conn)
        .await?
        .rows_affected())
    }

    async fn update(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Self::Output> {
        let _ = conn;
        todo!()
    }
}
