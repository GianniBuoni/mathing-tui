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

        // check if item exists
        ItemParams::new().with_item_id(item_id).get(conn).await?;

        if self.users.is_empty() {
            return Err(RequestError::missing_param(
                RequestType::Post,
                "joined receipt",
                "user id(s)",
            )
            .into());
        }

        let receipt = ReceiptParams::new()
            .item_id(item_id)
            .item_qty(item_qty)
            .post(&mut tx)
            .await?;

        for u_id in self.users.clone() {
            UserParams::new().with_user_id(u_id).get(conn).await?;
            ReceiptsUsersParams::new()
                .with_r_id(receipt.id)
                .with_u_id(u_id)
                .post(&mut tx)
                .await?;
        }
        tx.commit().await?;

        JoinedReceiptParams::default()
            .with_r_id(receipt.id)
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

        ReceiptParams::new().r_id(id).get(&mut tx).await?;

        let res = ReceiptParams::new().r_id(id).delete(&mut tx).await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn update(&self, conn: &SqlitePool) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Update)?;

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

        if self.item_id.is_some() || self.item_qty.is_some() {
            Into::<ReceiptParams>::into(self).update(&mut tx).await?;
        }

        // update users if user params are included
        if !self.users.is_empty() {
            // reset receipt_user rows
            let current_users = ReceiptsUsersParams::new()
                .with_r_id(id)
                .get(&mut tx)
                .await?
                .iter()
                .map(|f| f.user_id)
                .collect::<Vec<i64>>();

            for user in current_users {
                ReceiptsUsersParams::new()
                    .with_r_id(id)
                    .with_u_id(user)
                    .delete(&mut tx)
                    .await?;
            }
            // add all users back in
            for user in &self.users {
                ReceiptsUsersParams::new()
                    .with_u_id(*user)
                    .with_r_id(id)
                    .post(&mut tx)
                    .await?;
            }
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
