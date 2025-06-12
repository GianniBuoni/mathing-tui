use super::{errors::RequestError, *};

impl ItemParamsBuilder {
    pub fn item_id(mut self, id: i64) -> Self {
        self.item_id = ParamOption::new(id);
        self
    }
    pub fn item_name(mut self, name: impl ToString) -> Self {
        self.item_name = ParamOption::new(name.to_string());
        self
    }
    pub fn item_price(mut self, price: f64) -> Self {
        self.item_price = ParamOption::new(price);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn build(self) -> ItemParams {
        ItemParams {
            item_id: self.item_id.unwrap(),
            item_name: self.item_name.unwrap(),
            item_price: self.item_price.unwrap(),
            offset: self.offset,
        }
    }
}

impl ItemParams {
    pub fn builder() -> ItemParamsBuilder {
        ItemParamsBuilder::default()
    }
}

impl<'e> Request<'e> for ItemParams {
    type Output = StoreItem;
    type Connection = &'e SqlitePool;

    fn check_id(&self) -> Result<i64> {
        Ok(self.item_id.ok_or(RequestError::missing_param("id"))?)
    }

    async fn get_all(
        &self,
        conn: Self::Connection,
    ) -> Result<Vec<Self::Output>> {
        let offset =
            self.offset.ok_or(RequestError::missing_param("offset"))?;

        Ok(sqlx::query_as!(
            StoreItem,
            "SELECT * FROM items ORDER BY name LIMIT 20 OFFSET ?1",
            offset
        )
        .fetch_all(conn)
        .await?)
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let id = self.check_id()?;

        Ok(
            sqlx::query_as!(StoreItem, "SELECT * FROM items WHERE id=?1", id)
                .fetch_one(conn)
                .await
                .map_err(|_| RequestError::not_found(id, "items"))?,
        )
    }

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
        let name = self
            .item_name
            .clone()
            .ok_or(RequestError::missing_param("item name"))?;

        let price = self
            .item_price
            .ok_or(RequestError::missing_param("item price"))?;

        let now = get_time()?;

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
        let id = self.check_id()?;

        if self.item_name.is_none() && self.item_price.is_none() {
            return Err(
                RequestError::missing_param("item name, item price").into()
            );
        }

        let now = get_time()?;

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
        let id = self.check_id()?;

        let res = sqlx::query!("DELETE FROM items WHERE id=?1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(res.rows_affected())
    }
}
