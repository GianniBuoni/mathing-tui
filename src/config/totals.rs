use super::*;

impl StoreTotal {
    // setters
    pub async fn try_init(conn: &SqlitePool) -> Result<Mutex<Self>> {
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
    /// Calculates a new StoreTotal and replaces the current one
    /// with this new total.
    /// This method should only be called after the StoreTotal struct
    /// has been initialized elsewhere.
    pub async fn try_refresh(conn: &SqlitePool) -> Result<DbPayload> {
        let new_value = TotalsParams::get_total(conn).await?;
        // using try_get here to avoid potentailly initalizing the value
        // only to replace the value later in the function.
        let mut current = AppConfig::try_get_totals()?
            .lock()
            .map_err(|_| AppError::StoreTotalMutex)?;
        *current = new_value;

        Ok(DbPayload::None)
    }
    /// Returns value of specific value for a given key in StoreTotal.
    pub fn try_get_inner(key: i64) -> Result<Decimal> {
        let totals = AppConfig::try_get_totals()?
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
    // TODO: make into integration test now that its in new module?
    //use crate::db::params::tests::init_join_rows;
    use rust_decimal::dec;

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

    // #[sqlx::test]
    // async fn test_totals_adding(conn: SqlitePool) -> Result<()> {
    //     init_join_rows(&conn).await?;
    //     let want = expected_totals();
    //     let mut got = StoreTotal::default();

    //     JoinedReceiptParams::default()
    //         .with_offset(0)
    //         .get_all(&conn)
    //         .await?
    //         .into_iter()
    //         .zip(intermediate_totals())
    //         .try_for_each(|(row, want)| {
    //             anyhow::Ok({
    //                 assert_eq!(want, row.try_calc()?);
    //                 got.add(row.try_calc()?);
    //             })
    //         })?;

    //     assert_eq!(want, got.0, "Test if all the math is right ✨");

    //     Ok(())
    // }
}
