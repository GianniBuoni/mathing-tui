use anyhow::Ok;

use super::*;
use crate::prelude::*;

impl JoinedReceiptParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }
    pub fn item_id(mut self, item_id: i64) -> Self {
        self.item_id = Some(item_id);
        self
    }
    pub fn item_qty(mut self, item_qty: i64) -> Self {
        self.item_qty = Some(item_qty);
        self
    }
    pub fn add_user(mut self, u_id: i64) -> Self {
        self.users.push(u_id);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl<'e> Request<'e> for JoinedReceiptParams {
    type Output = StoreJoinRow;
    type Connection = &'e SqlitePool;

    fn check_id(&self) -> Result<i64> {
        Ok(self.r_id.ok_or(RequestError::missing_param("receipt id"))?)
    }

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;

        let item_id =
            self.item_id.ok_or(RequestError::missing_param("item id"))?;
        let item_qty = self
            .item_qty
            .ok_or(RequestError::missing_param("item qty"))?;

        // check if item exists
        ItemParams::new().item_id(item_id).get(conn).await?;

        if self.users.is_empty() {
            return Err(RequestError::missing_param("user id(s)").into());
        }

        let receipt = ReceiptParams::new()
            .item_id(item_id)
            .item_qty(item_qty)
            .post(&mut *tx)
            .await?;

        for u_id in self.users.clone() {
            UserParams::new().user_id(u_id).get(conn).await?;
            ReceiptsUsersParams::new()
                .r_id(receipt.id)
                .u_id(u_id)
                .post(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(JoinedReceiptParams::new()
            .r_id(receipt.id)
            .get(conn)
            .await?)
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let r_id = self.check_id()?;
        let offset = if self.offset.is_some() {
            self.offset.unwrap()
        } else {
            0
        };

        let raw_rows = sqlx::query_file_as!(
            StoreJoinRaw,
            "sql/get_ru_single.sql",
            r_id,
            offset,
        )
        .fetch_one(conn)
        .await?;

        Ok(raw_rows.as_join_row(conn).await?)
    }

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let r_id = self.check_id()?;

        let res = ReceiptParams::new().r_id(r_id).delete(&mut *tx).await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
        let _ = conn;
        todo!()
    }
}
