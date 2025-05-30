use super::{errors::RequestError, *};

impl ReceiptParams {
    pub fn new() -> Self {
        ReceiptParams::default()
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
}

impl Request<StoreReceipt> for ReceiptParams {
    fn check_id(&self) -> Result<i64> {
        Ok(self.r_id.ok_or(RequestError::missing_param("id"))?)
    }

    async fn get(&self, conn: &mut SqliteConnection) -> Result<StoreReceipt> {
        let id = self.check_id()?;
        Ok(sqlx::query_as!(
            StoreReceipt,
            "SELECT * FROM receipts WHERE id=?1",
            id
        )
        .fetch_one(conn)
        .await?)
    }

    async fn post(&self, conn: &mut SqliteConnection) -> Result<StoreReceipt> {
        let item_id =
            self.item_id.ok_or(RequestError::missing_param("item id"))?;
        let qty = self
            .item_qty
            .ok_or(RequestError::missing_param("item qty"))?;
        let now = get_time()?;

        Ok(sqlx::query_as!(
            StoreReceipt,
            "
        INSERT INTO receipts (
            created_at, updated_at, item_id, item_qty
        ) VALUES (
            ?1, ?2, ?3, ?4
        ) RETURNING *
        ",
            now,
            now,
            item_id,
            qty,
        )
        .fetch_one(conn)
        .await?)
    }

    async fn delete(&self, conn: &mut SqliteConnection) -> Result<u64> {
        let id = self.check_id()?;

        Ok(sqlx::query!("DELETE FROM receipts WHERE id=?1", id)
            .execute(conn)
            .await?
            .rows_affected())
    }

    async fn update(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<StoreReceipt> {
        let id = self.check_id()?;
        if self.item_id.is_none() && self.item_qty.is_none() {
            return Err(
                RequestError::missing_param("item id and item qty").into()
            );
        }

        if let Some(item_id) = self.item_id {
            sqlx::query!(
                "UPDATE receipts SET item_id=?1 WHERE id=?2",
                item_id,
                id
            )
            .execute(&mut *conn)
            .await?;
        }

        if let Some(item_qty) = self.item_qty {
            sqlx::query!(
                "UPDATE receipts SET item_qty=?1 WHERE id=?2",
                item_qty,
                id
            )
            .execute(&mut *conn)
            .await?;
        }

        let now = get_time()?;
        sqlx::query!("UPDATE receipts SET updated_at=?1 WHERE id=?2", now, id)
            .execute(&mut *conn)
            .await?;

        self.get(conn).await
    }
}
