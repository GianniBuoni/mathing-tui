use sqlx::SqliteConnection;

use super::{errors::RequestError, *};

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

    fn check_id(&self) -> Result<i64> {
        Ok(self.r_id.ok_or(RequestError::missing_param("receipt id"))?)
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let id = self.check_id()?;

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
        let r_id = self.check_id()?;
        let u_id = self.u_id.ok_or(RequestError::missing_param("user id"))?;
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
        let id = self.check_id()?;
        let u_id = self.u_id.ok_or(RequestError::missing_param("user id"))?;

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
