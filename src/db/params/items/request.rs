use super::*;

impl<'e> Request<'e> for ItemParams {
    type Output = StoreItem;
    type Connection = &'e SqlitePool;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.item_id
            .ok_or(RequestError::missing_param(req_type, "item", "id"))
    }

    async fn get_all(
        &self,
        conn: Self::Connection,
    ) -> Result<Vec<Self::Output>> {
        let offset = self.offset.ok_or(RequestError::missing_param(
            RequestType::GetAll,
            "items",
            "offset",
        ))?;

        Ok(sqlx::query_as!(
            StoreItem,
            "SELECT * FROM items ORDER BY name LIMIT 20 OFFSET ?1",
            offset
        )
        .fetch_all(conn)
        .await?)
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Get)?;

        Ok(
            sqlx::query_as!(StoreItem, "SELECT * FROM items WHERE id=?1", id)
                .fetch_one(conn)
                .await
                .map_err(|_| RequestError::not_found(id, "items"))?,
        )
    }

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
        let name = self.item_name.clone().ok_or(
            RequestError::missing_param(RequestType::Post, "item", "item name"),
        )?;

        let price = self.item_price.ok_or(RequestError::missing_param(
            RequestType::Post,
            "item",
            "item price",
        ))?;

        let now = DbConn::try_get_time()?;

        Ok(sqlx::query_as!(
            StoreItem,
            "
            INSERT INTO items (
                name, price, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4
            ) RETURNING *
            ",
            name,
            price,
            now,
            now,
        )
        .fetch_one(conn)
        .await?)
    }

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Update)?;

        if self.item_name.is_none() && self.item_price.is_none() {
            return Err(RequestError::missing_param(
                RequestType::Update,
                "item",
                "item name, item price",
            )
            .into());
        }
        let now = DbConn::try_get_time()?;

        if let Some(name) = self.item_name.clone() {
            sqlx::query!("UPDATE items SET name=?1 WHERE id=?2", name, id)
                .execute(&mut *tx)
                .await?;
        }

        if let Some(price) = self.item_price {
            sqlx::query!("UPDATE items SET price=?1 WHERE id=?2", price, id)
                .execute(&mut *tx)
                .await?;
        };

        sqlx::query!("UPDATE items SET updated_at=?1 WHERE id=?2", now, id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        self.get(conn).await
    }

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Delete)?;

        let res = sqlx::query!("DELETE FROM items WHERE id=?1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(res.rows_affected())
    }
}
