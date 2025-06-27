use super::*;

impl<'e> Request<'e> for JoinedReceiptParams {
    type Output = StoreJoinRow;
    type Connection = &'e SqlitePool;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError> {
        self.r_id.ok_or(RequestError::missing_param(
            req_type,
            "joined receipt",
            "receipt id",
        ))
    }

    async fn get_all(
        &self,
        conn: Self::Connection,
    ) -> Result<Vec<Self::Output>> {
        let offset = self.offset.ok_or(RequestError::missing_param(
            RequestType::GetAll,
            "joined receipt",
            "offset",
        ))?;

        let raw = sqlx::query_file_as!(
            StoreJoinRaw,
            "sql/get_receipts_users.sql",
            offset
        )
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

    async fn post(&self, conn: Self::Connection) -> Result<Self::Output> {
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
            .post(&mut *tx)
            .await?;

        for u_id in self.users.clone() {
            UserParams::new().with_user_id(u_id).get(conn).await?;
            ReceiptsUsersParams::new()
                .with_r_id(receipt.id)
                .with_u_id(u_id)
                .post(&mut *tx)
                .await?;
        }
        tx.commit().await?;

        JoinedReceiptParams::new()
            .with_r_id(receipt.id)
            .get(conn)
            .await
    }

    async fn get(&self, conn: Self::Connection) -> Result<Self::Output> {
        let r_id = self.check_id(RequestType::Get)?;
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
        .await
        .map_err(|_| {
            RequestError::not_found(r_id, "receipts_users (joined)")
        })?;

        raw_rows.try_join_row(conn).await
    }

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let id = self.check_id(RequestType::Delete)?;

        ReceiptParams::new().r_id(id).get(&mut *tx).await?;

        let res = ReceiptParams::new().r_id(id).delete(&mut *tx).await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
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
            Into::<ReceiptParams>::into(self).update(&mut *tx).await?;
        }

        // update users if user params are included
        if !self.users.is_empty() {
            // reset receipt_user rows
            let current_users = ReceiptsUsersParams::new()
                .with_r_id(id)
                .get(&mut *tx)
                .await?
                .iter()
                .map(|f| f.user_id)
                .collect::<Vec<i64>>();

            for user in current_users {
                ReceiptsUsersParams::new()
                    .with_r_id(id)
                    .with_u_id(user)
                    .delete(&mut *tx)
                    .await?;
            }
            // add all users back in
            for user in &self.users {
                ReceiptsUsersParams::new()
                    .with_u_id(*user)
                    .with_r_id(id)
                    .post(&mut *tx)
                    .await?;
            }
        }
        tx.commit().await?;

        JoinedReceiptParams::new().with_r_id(id).get(conn).await
    }
}
