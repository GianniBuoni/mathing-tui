use anyhow::Ok;

use super::*;

impl JoinParamsBuilder {
    pub fn r_id(mut self, r_id: i64) -> Self {
        self.r_id = ParamOption::new(r_id);
        self
    }
    pub fn item_id(mut self, item_id: i64) -> Self {
        self.item_id = ParamOption::new(item_id);
        self
    }
    pub fn item_qty(mut self, item_qty: i64) -> Self {
        self.item_qty = ParamOption::new(item_qty);
        self
    }
    pub fn add_user(self, u_id: i64) -> Self {
        let users = self.users.clone();
        {
            let mut users = users.borrow_mut();
            users.push(u_id);
        }
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn build(&self) -> JoinedReceiptParams {
        let users = self.users.clone();
        let users = users.borrow().to_owned();

        JoinedReceiptParams {
            offset: self.offset,
            users,
            r_id: self.r_id.unwrap(),
            item_id: self.item_id.unwrap(),
            item_qty: self.item_qty.unwrap(),
        }
    }
}

impl JoinedReceiptParams {
    pub fn builder() -> JoinParamsBuilder {
        JoinParamsBuilder::default()
    }
    fn new() -> Self {
        Self::default()
    }
    fn r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }
    pub async fn reset(&self, conn: &SqlitePool) -> Result<u64> {
        Ok(sqlx::query!("DELETE FROM receipts")
            .execute(conn)
            .await?
            .rows_affected())
    }
}

impl<'e> Request<'e> for JoinedReceiptParams {
    type Output = StoreJoinRow;
    type Connection = &'e SqlitePool;

    fn check_id(&self) -> Result<i64> {
        Ok(self.r_id.ok_or(RequestError::missing_param("receipt id"))?)
    }

    async fn get_all(
        &self,
        conn: Self::Connection,
    ) -> Result<Vec<Self::Output>> {
        let offset =
            self.offset.ok_or(RequestError::missing_param("offset"))?;

        let raw = sqlx::query_file_as!(
            StoreJoinRaw,
            "sql/get_receipts_users.sql",
            offset
        )
        .fetch_all(conn)
        .await?;

        Ok(try_join_all(
            raw.iter().map(async |raw| raw.as_join_row(conn).await),
        )
        .await?
        .into_iter()
        .collect())
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
        .await
        .map_err(|_| {
            RequestError::not_found(r_id, "receipts_users (joined)")
        })?;

        Ok(raw_rows.as_join_row(conn).await?)
    }

    async fn delete(&self, conn: Self::Connection) -> Result<u64> {
        let mut tx = conn.begin().await?;
        let id = self.check_id()?;

        ReceiptParams::new().r_id(id).get(&mut *tx).await?;

        let res = ReceiptParams::new().r_id(id).delete(&mut *tx).await?;
        tx.commit().await?;

        Ok(res)
    }

    async fn update(&self, conn: Self::Connection) -> Result<Self::Output> {
        let mut tx = conn.begin().await?;
        let id = self.check_id()?;

        if self.item_id.is_none()
            && self.item_qty.is_none()
            && self.users.is_empty()
        {
            return Err(RequestError::missing_param(
                "item id, item qty, or users",
            )
            .into());
        }

        if self.item_id.is_some() || self.item_qty.is_some() {
            Into::<ReceiptParams>::into(self).update(&mut *tx).await?;
        }

        if !self.users.is_empty() {
            let ru = ReceiptsUsersParams::new()
                .r_id(id)
                .get(&mut *tx)
                .await?
                .iter()
                .map(|ru| ru.user_id)
                .collect::<Vec<i64>>();

            for u_id in ru {
                if !self.users.clone().contains(&u_id) {
                    ReceiptsUsersParams::new()
                        .r_id(id)
                        .u_id(u_id)
                        .delete(&mut *tx)
                        .await?;
                }
            }
        }
        tx.commit().await?;

        Ok(JoinedReceiptParams::new().r_id(id).get(conn).await?)
    }
}
