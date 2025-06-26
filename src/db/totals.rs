use std::sync::Mutex;
use tokio::sync::OnceCell;

use super::*;

static TOTALS: OnceCell<Mutex<StoreTotal>> = OnceCell::const_new();

impl StoreTotal {
    // setters
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
    pub fn add(&mut self, other: HashMap<i64, Decimal>) {
        other.into_iter().for_each(|(key, val)| {
            self.0.entry(key).and_modify(|e| *e += val).or_insert(val);
        });
    }
    pub fn subtract(&mut self, other: HashMap<i64, Decimal>) {
        other.into_iter().for_each(|(key, val)| {
            self.0.entry(key).and_modify(|f| *f -= val);
        });
    }
    // getters
    /// Returns the StoreTotal Mutex. Errors out if there the OnecCell
    /// variable hasn't been initialized yet.
    pub fn try_get() -> Result<&'static Mutex<Self>> {
        TOTALS
            .get()
            .ok_or(ComponentError::not_found("StoreTotal Mutex").into())
    }
    /// Returns the StoreTotal Mutex. Initializes the OnceCell if there is
    /// no value contained within.
    pub async fn get_or_init() -> Result<&'static Mutex<Self>> {
        TOTALS.get_or_try_init(Self::new).await
    }
    /// Returns value of specific value for a given key in StoreTotal.
    pub fn try_get_inner(key: i64) -> Result<Decimal> {
        let message =
            "Mutex error: Current thread can't obtain lock for StoreTotal.";
        let totals = Self::try_get()?
            .lock()
            .map_err(|_| anyhow::Error::msg(message))?;

        totals
            .0
            .get(&key)
            .copied()
            .ok_or(anyhow::Error::msg("No total found for given key."))
    }
}
