use super::*;

impl StoreTotal {
    // setters
    pub(super) async fn try_init(conn: &SqlitePool) -> Result<Mutex<Self>> {
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
        Ok(&CONFIG.get().ok_or(AppError::ConfigInit)?.totals)
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
    /// Sum up values in the StoreTotal hashmap
    pub fn sum_total(&self, rows: &mut Vec<Row>) {
        let res = self
            .0
            .values()
            .fold(Decimal::default(), |acc, values| acc + *values);
        let row = Row::new([
            Cell::from(" TOTAL").bold(),
            Cell::from(format!(" {:.2}", res)),
        ]);
        rows.push(row);
    }
}

impl From<HashMap<i64, Decimal>> for StoreTotal {
    fn from(value: HashMap<i64, Decimal>) -> Self {
        Self(value)
    }
}
