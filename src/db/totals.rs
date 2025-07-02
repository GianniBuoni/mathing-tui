use std::sync::Mutex;

use super::*;

pub mod prelude {
    pub use super::StoreTotal;
}

static TOTALS: OnceCell<Mutex<StoreTotal>> = OnceCell::const_new();

#[derive(Debug, Default, PartialEq)]
pub struct StoreTotal(HashMap<i64, Decimal>);

impl StoreTotal {
    // setters
    async fn init() -> Result<Mutex<Self>> {
        let conn = DbConn::try_get().await?;
        let total = TotalsParams::get_total(conn).await?;
        Ok(Mutex::new(total))
    }
    /// Takes a hashmap and adds Decimal values to the StoreTotal
    /// if a key already exists.
    /// Adds a key to the underlying StoreTotal hashmap
    /// if said key does not already extist.
    pub fn add(&mut self, other: HashMap<i64, Decimal>) {
        other.into_iter().for_each(|(key, val)| {
            self.0.entry(key).and_modify(|e| *e += val).or_insert(val);
        });
    }
    /// Takes a hashmap and subtracts Decimal values
    /// from the StoreTotal if a key in the provided hashmap
    /// already exists in the Storetotal's underlying hashmap.
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
    pub async fn get_or_try_init() -> Result<&'static Mutex<Self>> {
        TOTALS.get_or_try_init(Self::init).await
    }
    /// Calculates a new StoreTotal and replaces the current one
    /// with this new total.
    /// This method should only be called after the StoreTotal struct
    /// has been initialized elsewhere.
    pub async fn try_refresh(conn: &SqlitePool) -> Result<DbPayload> {
        let new_value = TotalsParams::get_total(conn).await?;
        // using try_get here to avoid potentailly initalizing the value
        // only to replace the value later in the function.
        let mut current = Self::try_get()?
            .lock()
            .map_err(|_| AppError::StoreTotalMutex)?;
        *current = new_value;

        Ok(DbPayload::None)
    }
    /// Returns value of specific value for a given key in StoreTotal.
    pub fn try_get_inner(key: i64) -> Result<Decimal> {
        let totals = Self::try_get()?
            .lock()
            .map_err(|_| AppError::StoreTotalMutex)?;

        totals
            .0
            .get(&key)
            .copied()
            .ok_or(AppError::StoreTotalKey(key).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::params::tests::init_join_rows;

    fn intermediate_totals() -> Vec<HashMap<i64, Decimal>> {
        vec![
            HashMap::from([(3, dec!(9.98))]),
            HashMap::from([(2, dec!(9.49))]),
            HashMap::from([(2, dec!(8.32)), (3, dec!(8.32))]),
        ]
    }
    fn expected_totals() -> HashMap<i64, Decimal> {
        HashMap::from([(3, dec!(18.30)), (2, dec!(17.81))])
    }

    #[sqlx::test]
    async fn test_totals_adding(conn: SqlitePool) -> Result<()> {
        init_join_rows(&conn).await?;
        let want = expected_totals();
        let mut got = StoreTotal::default();

        JoinedReceiptParams::builder()
            .with_offset(0)
            .build()
            .get_all(&conn)
            .await?
            .into_iter()
            .zip(intermediate_totals())
            .try_for_each(|(row, want)| {
                anyhow::Ok({
                    assert_eq!(want, row.try_calc()?);
                    got.add(row.try_calc()?);
                })
            })?;

        assert_eq!(want, got.0, "Test if all the math is right ✨");

        Ok(())
    }
}
