use sqlx::SqliteConnection;

use super::*;

impl ReceiptsUsersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }

    pub fn u_id(mut self, u_id: i64) -> Self {
        self.u_id = Some(u_id);
        self
    }
}

impl<'e> Request<'e> for ReceiptsUsersParams {
    type Output = Vec<StoreReceiptsUsers>;
    type Connection = &'e mut SqliteConnection;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.r_id.ok_or(RequestError::missing_param(
            req_type,
            "receipts users",
            "receipt id",
        ))
    }

    /// get_all for Receipt Params should not be called directly
    /// consider getting data needed from [`JoinedReceiptsParams`]
    /// instead
    async fn get_all(
        &self,
        conn: Self::Connection,
    ) -> Result<Vec<Self::Output>> {
        let _ = conn;
        todo!()
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
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

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
        let r_id = self.check_id(RequestType::Post)?;

        let u_id = self.u_id.ok_or(RequestError::missing_param(
            RequestType::Post,
            "receipts users",
            "user id",
        ))?;
        let now = get_time()?;

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

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
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

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
        let _ = conn;
        todo!()
    }
}
