use super::*;

impl TotalsParams {
    pub(super) async fn get_prices(
        conn: &SqlitePool,
    ) -> Result<Vec<StoreJoinPrices>> {
        Ok({
            sqlx::query_as!(
                StoreJoinPrices,
                "
SELECT
  GROUP_CONCAT( CAST(ru.user_id as TEXT)) as user_ids,
  COUNT(ru.user_id) as user_count,
  i.price as item_price,
  r.item_qty as item_qty
FROM receipts_users ru
INNER JOIN users u ON ru.user_id= u.id
INNER JOIN receipts r ON ru.receipt_id = r.id
INNER JOIN items i ON r.item_id = i.id
GROUP BY ru.receipt_id
               "
            )
            .fetch_all(conn)
            .await?
        })
    }
    pub async fn get_total(conn: &SqlitePool) -> Result<StoreTotal> {
        Self::get_prices(conn).await?.into_iter().try_fold(
            StoreTotal::default(),
            |mut acc, next| {
                acc.add(next.try_calc()?);
                Aok(acc)
            },
        )
    }
}
