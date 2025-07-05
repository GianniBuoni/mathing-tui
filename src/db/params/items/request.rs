use super::*;

impl Request for ItemParams {
    type Output = StoreItem;
    type Outputs = Vec<StoreItem>;

    fn get_app_arm(&self) -> AppArm {
        AppArm::Items
    }

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.item_id
            .ok_or(RequestError::missing_param(req_type, "item", "id"))
    }

    async fn get_all(&self, conn: &SqlitePool) -> Result<Self::Outputs> {
        let mut query = QueryBuilder::<Sqlite>::new("SELECT * FROM items");

        if let Some(search) = self.search_filter.as_ref() {
            query
                .push(" WHERE name LIKE concat('%', ")
                .push_bind(search)
                .push(", '%')");
        }
        let limit = self.limit.unwrap_or(20);
        query.push(" ORDER BY name LIMIT ").push_bind(limit);

        if let Some(offset) = self.offset.as_ref() {
            query.push(" OFFSET ").push_bind(offset);
        }
        Ok(query.build_query_as().fetch_all(conn).await?)
    }

    async fn get(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let id = self.check_id(RequestType::Get)?;

        Ok(
            sqlx::query_as!(StoreItem, "SELECT * FROM items WHERE id=?1", id)
                .fetch_one(conn)
                .await
                .map_err(|_| RequestError::not_found(id, "items"))?,
        )
    }

    async fn post(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let name = self.item_name.clone().ok_or(
            RequestError::missing_param(RequestType::Post, "item", "item name"),
        )?;

        let price = self.item_price.ok_or(RequestError::missing_param(
            RequestType::Post,
            "item",
            "item price",
        ))?;

        let now = AppConfig::try_get_time()?;

        Ok(sqlx::query_as!(
            StoreItem,
            "INSERT INTO items (
                name, price, created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4
            ) RETURNING *",
            name,
            price,
            now,
            now,
        )
        .fetch_one(conn)
        .await?)
    }

    async fn update(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Update)?;
        let now = AppConfig::try_get_time()?;

        if self.item_name.is_none() && self.item_price.is_none() {
            return Err(RequestError::missing_param(
                RequestType::Update,
                "item",
                "item name, item price",
            )
            .into());
        }
        let mut query =
            QueryBuilder::<Sqlite>::new("UPDATE items SET updated_at=");

        query.push_bind(now);

        if let Some(name) = self.item_name.as_ref() {
            query.push(", name=").push_bind(name);
        }
        if let Some(price) = self.item_price {
            query.push(", price=").push_bind(price);
        };

        query
            .push(" WHERE id=")
            .push_bind(id)
            .build()
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        self.get(conn).await
    }

    async fn delete(&self, conn: &SqlitePool) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Delete)?;

        let res = sqlx::query!("DELETE FROM items WHERE id=?1", id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(res.rows_affected())
    }

    async fn count(&self, conn: &SqlitePool) -> Result<i64> {
        Ok(
            sqlx::query_as!(StoreCount, "SELECT COUNT(*) AS rows FROM items")
                .fetch_one(conn)
                .await
                .unwrap_or_default()
                .rows,
        )
    }
}
