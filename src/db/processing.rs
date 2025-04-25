use futures::future::try_join_all;

use super::*;

impl StoreJoinRaw {
    pub async fn as_join_row(
        &self,
        conn: &SqlitePool,
    ) -> Result<StoreJoinRow, Box<dyn Error>> {
        // get ids
        let users = try_join_all(self.user_ids.split(",").map(async |s| {
            Ok::<StoreUser, Box<dyn Error>>({
                let id = s.parse::<i64>()?;
                get_store_user_single(conn, id).await?
            })
        }))
        .await?;

        Ok(StoreJoinRow {
            users,
            receipt_id: self.receipt_id,
            item_name: self.item_name.clone(),
            item_id: self.item_id,
            item_price: self.item_price,
            item_qty: self.item_qty,
            user_count: self.user_count,
        })
    }
}
