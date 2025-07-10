use super::*;

impl Request for JoinedReceiptParams {
    type Output = StoreJoinRow;
    type Outputs = Vec<StoreJoinRow>;

    fn get_app_arm(&self) -> AppArm {
        AppArm::Receipts
    }

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.r_id.ok_or(RequestError::missing_param(
            req_type,
            "joined receipt",
            "receipt id",
        ))
    }

    async fn get_all(&self, conn: &SqlitePool) -> Result<Self::Outputs> {
        let mut q = QueryBuilder::<Sqlite>::new(JOIN_QUERY_BASE);
        let limit = self.limit.unwrap_or(20);
        let offset = self.offset.unwrap_or_default();

        let raw: Vec<StoreJoinRaw> = q
            .push(" GROUP BY ru.receipt_id LIMIT ")
            .push_bind(limit)
            .push(" OFFSET ")
            .push_bind(offset)
            .build_query_as()
            .fetch_all(conn)
            .await?;
        Ok(try_join_all(
            raw.into_iter()
                .map(async |raw| raw.try_join_row(conn).await),
        )
        .await?
        .into_iter()
        .collect())
    }

    async fn post(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let now = AppConfig::try_get_time()?;

        let item_id = self.item_id.ok_or(RequestError::missing_param(
            RequestType::Post,
            "joined receipt",
            "item id",
        ))?;
        let item_qty = self.item_qty.ok_or(RequestError::missing_param(
            RequestType::Post,
            "joined receipt",
            "item qty",
        ))?;

        if self.users.is_empty() {
            return Err(RequestError::missing_param(
                RequestType::Post,
                "joined receipt",
                "user id(s)",
            )
            .into());
        }
        // check if item exists
        ItemParams::default()
            .with_item_id(item_id)
            .get(conn)
            .await?;

        let r = sqlx::query_as!(
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
            item_qty,
        )
        .fetch_one(&mut *tx)
        .await?;

        let ru_params = self
            .users
            .iter()
            .map(|f| (*f, r.id))
            .collect::<Vec<(i64, i64)>>();

        let mut q = QueryBuilder::new(
            "INSERT INTO receipts_users (
                created_at, updated_at, receipt_id, user_id
            ) ",
        );
        q.push_values(ru_params, |mut q, (u_id, r_id)| {
            q.push_bind(now)
                .push_bind(now)
                .push_bind(r_id)
                .push_bind(u_id);
        })
        .build()
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        JoinedReceiptParams::default()
            .with_r_id(r.id)
            .get(conn)
            .await
    }

    async fn get(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let r_id = self.check_id(RequestType::Get)?;
        let mut q = QueryBuilder::<Sqlite>::new(JOIN_QUERY_BASE);

        let mut raw: Vec<StoreJoinRaw> = q
            .push(" WHERE ru.receipt_id =")
            .push_bind(r_id)
            .push(" GROUP BY ru.receipt_id")
            .build_query_as()
            .fetch_all(conn)
            .await?;
        // check if query returned an empy Vec
        if raw.is_empty() {
            return Err(RequestError::not_found(
                r_id,
                "receipts_users (joined)",
            )
            .into());
        }
        raw.remove(0).try_join_row(conn).await
    }

    async fn delete(&self, conn: &SqlitePool) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Delete)?;

        // check if exists
        sqlx::query!("SELECT * from receipts WHERE id=?", id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| RequestError::not_found(id, "receipts"))?;

        let res = sqlx::query!("DELETE FROM receipts WHERE id=?", id)
            .execute(&mut *tx)
            .await?
            .rows_affected();
        tx.commit().await?;

        Ok(res)
    }

    async fn update(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Update)?;
        let now = AppConfig::try_get_time()?;

        if self.item_id.is_none()
            && self.item_qty.is_none()
            && self.users.is_empty()
        {
            return Err(RequestError::missing_param(
                RequestType::Update,
                "joined receipt",
                "item id, item qty, or users",
            )
            .into());
        }
        // check if exists
        sqlx::query!("SELECT * FROM receipts WHERE id=?", id)
            .fetch_one(conn)
            .await
            .map_err(|_| RequestError::not_found(id, "receipts"))?;

        // update receipt
        let mut q = QueryBuilder::<Sqlite>::new("UPDATE receipts SET");

        if let Some(item_id) = self.item_id {
            q.push(" item_id=").push_bind(item_id).push(",");
        }
        if let Some(item_qty) = self.item_qty {
            q.push(" item_qty=").push_bind(item_qty).push(",");
        }
        q.push(" updated_at=")
            .push_bind(now)
            .push(" WHERE id=")
            .push_bind(id);
        dbg!(q.sql());
        q.build().execute(&mut *tx).await?;

        // update receipts users
        if !self.users.is_empty() {
            // check if receits_users exist
            // reset receipt_user rows
            sqlx::query!("DELETE from receipts_users WHERE receipt_id=?1", id)
                .execute(&mut *tx)
                .await?;

            // re-add to receipts users
            let mut q = QueryBuilder::<Sqlite>::new(
                "INSERT into receipts_users (created_at, updated_at, receipt_id, user_id)",
            );
            q.push_values(self.users.iter(), |mut q, user| {
                q.push_bind(now)
                    .push_bind(now)
                    .push_bind(id)
                    .push_bind(user);
            })
            .build()
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        JoinedReceiptParams::default().with_r_id(id).get(conn).await
    }

    async fn count(&self, conn: &SqlitePool) -> Result<i64> {
        Ok(sqlx::query_as!(
            StoreCount,
            "SELECT
                COUNT(DISTINCT receipt_id) AS rows
            FROM receipts_users"
        )
        .fetch_one(conn)
        .await
        .unwrap_or_default()
        .rows)
    }

    async fn reset(&self, conn: &SqlitePool) -> Result<u64> {
        Ok(sqlx::query!("DELETE FROM receipts")
            .execute(conn)
            .await?
            .rows_affected())
    }
}
