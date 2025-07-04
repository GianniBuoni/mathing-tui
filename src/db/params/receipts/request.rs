use super::*;

impl Transaction for ReceiptParams {
    type Output = StoreReceipt;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.r_id
            .ok_or(RequestError::missing_param(req_type, "receipt", "id"))
    }

    async fn get(&self, conn: &mut SqliteConnection) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Get)?;

        Ok(sqlx::query_as!(
            StoreReceipt,
            "SELECT * FROM receipts WHERE id=?1",
            id
        )
        .fetch_one(conn)
        .await
        .map_err(|_| RequestError::not_found(id, "receipts"))?)
    }

    async fn post(&self, conn: &mut SqliteConnection) -> Result<Self::Output> {
        let item_id = self.item_id.ok_or(RequestError::missing_param(
            RequestType::Post,
            "receipt",
            "item id",
        ))?;
        let qty = self.item_qty.ok_or(RequestError::missing_param(
            RequestType::Post,
            "receipt",
            "item qty",
        ))?;
        let now = DbConn::try_get_time()?;

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
        let id = self.check_id(RequestType::Delete)?;

        Ok(sqlx::query!("DELETE FROM receipts WHERE id=?1", id)
            .execute(conn)
            .await?
            .rows_affected())
    }

    async fn update(
        &self,
        conn: &mut SqliteConnection,
    ) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Delete)?;

        if self.item_id.is_none() && self.item_qty.is_none() {
            return Err(RequestError::missing_param(
                RequestType::Update,
                "receipt",
                "item id and item qty",
            )
            .into());
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
        let now = DbConn::try_get_time()?;

        sqlx::query!("UPDATE receipts SET updated_at=?1 WHERE id=?2", now, id)
            .execute(&mut *conn)
            .await?;

        self.get(conn).await
    }
}
