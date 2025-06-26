use std::sync::Mutex;
use tokio::sync::OnceCell;

use super::*;

static TOTALS: OnceCell<Mutex<StoreTotal>> = OnceCell::const_new();

impl StoreTotal {
    async fn new() -> Result<Mutex<Self>> {
        let conn = get_db().await?;

        let rows_to_calc =
            sqlx::query_file_as!(StoreJoinRaw, "sql/get_ru_no_offset.sql")
                .fetch_all(conn)
                .await
                .map_err(|_| RequestError::not_found("all", "recipts_users"))?;

        let calcs = try_join_all(rows_to_calc.into_iter().map(async |raw| {
            anyhow::Ok::<StoreJoinRow>(raw.as_join_row(conn).await?)
        }))
        .await?
        .iter_mut()
        .try_fold(StoreTotal::default(), |mut acc, next| {
            anyhow::Ok::<StoreTotal>({
                acc.add(next.calc()?);
                acc
            })
        })
        .unwrap_or_default();

        Ok(Mutex::new(calcs))
    }

    pub async fn get_or_init() -> Result<&'static Mutex<Self>> {
        TOTALS.get_or_try_init(Self::new).await
    }

    pub fn get_inner(key: i64) -> Result<Decimal> {
        let totals = TOTALS
            .get()
            .ok_or(ComponentError::not_found("StoreTotal Mutex"))?
            .lock()
            .map_err(|_| anyhow::Error::msg("Read lock failed for TOTALS"))?;

        totals
            .0
            .get(&key)
            .copied()
            .ok_or(anyhow::Error::msg("No total found for given key."))
    }

    pub fn get() -> Result<&'static Mutex<Self>> {
        TOTALS
            .get()
            .ok_or(ComponentError::not_found("StoreTotal Mutex").into())
    }
}
