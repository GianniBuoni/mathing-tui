use super::{errors::RequestError, *};

impl<'db> ItemParams<'db> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn item_id(mut self, id: i64) -> Self {
        self.item_id = Some(id);
        self
    }
    pub fn item_name(mut self, name: impl Into<Cow<'db, str>>) -> Self {
        self.item_name = Some(name.into());
        self
    }
    pub fn item_price(mut self, price: f64) -> Self {
        self.item_price = Some(price);
        self
    }
}

impl Request<StoreItem> for ItemParams<'_> {
    fn check_id(&self) -> Result<i64> {
        Ok(self.item_id.ok_or(RequestError::missing_param("id"))?)
    }

    async fn get(&self, conn: &mut SqliteConnection) -> Result<StoreItem> {
        let id = self.check_id()?;

        Ok(
            sqlx::query_as!(StoreItem, "SELECT * FROM items WHERE id=?1", id)
                .fetch_one(conn)
                .await?,
        )
    }

    async fn post(&self, conn: &mut SqliteConnection) -> Result<StoreItem> {
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

    async fn update(&self, conn: &mut SqliteConnection) -> Result<StoreItem> {
        let id = self.check_id()?;

        if self.item_name.is_none() && self.item_price.is_none() {
            return Err(
                RequestError::missing_param("item name, item price").into()
            );
        }

        let now = get_time()?;

        if let Some(name) = self.item_name.clone() {
            sqlx::query!("UPDATE items SET name=?1 WHERE id=?2", name, id)
                .execute(&mut *conn)
                .await?;
        }

        if let Some(price) = self.item_price {
            sqlx::query!("UPDATE items SET price=?1 WHERE id=?2", price, id)
                .execute(&mut *conn)
                .await?;
        };

        sqlx::query!("UPDATE items SET updated_at=?1 WHERE id=?2", now, id)
            .execute(&mut *conn)
            .await?;

        Ok(self.get(conn).await?)
    }

    async fn delete(&self, conn: &mut SqliteConnection) -> Result<()> {
        let id = self.check_id()?;

        sqlx::query!("DELETE FROM items WHERE id=?1", id)
            .execute(conn)
            .await?;

        Ok(())
    }
}
