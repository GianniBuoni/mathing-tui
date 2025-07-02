use super::*;

impl StoreJoinRaw {
    pub async fn try_join_row(self, conn: &SqlitePool) -> Result<StoreJoinRow> {
        let users = try_join_all(self.user_ids.split(",").map(async |s| {
            Aok::<StoreUser>({
                let id = s.parse::<i64>()?;
                UserParams::new().with_user_id(id).get(conn).await?
            })
        }))
        .await?;

        Ok(StoreJoinRow {
            users,
            receipt_id: self.receipt_id,
            item_name: self.item_name,
            item_id: self.item_id,
            item_price: self.item_price,
            item_qty: self.item_qty,
            user_count: self.user_count,
        })
    }
}

fn calc_decimal(price: f64, qty: i64, users: i64) -> Result<Decimal> {
    if users == 0 {
        // avoid divide by zero
        return Ok(dec!(0));
    }
    let err = "Decimal Error: Could not convert float to Decimal";
    Ok(Decimal::from_f64(price * qty as f64 / users as f64)
        .ok_or(Error::msg(err))?
        .round_dp(2))
}

impl StoreJoinPrices {
    pub fn try_calc(&self) -> Result<HashMap<i64, Decimal>> {
        let total =
            calc_decimal(self.item_price, self.item_qty, self.user_count)?;

        self.user_ids
            .split(",")
            .map(|f| {
                Aok::<(i64, Decimal)>({
                    let id = f.parse::<i64>()?;
                    (id, total)
                })
            })
            .collect()
    }
}

impl StoreJoinRow {
    pub fn try_calc(&self) -> Result<HashMap<i64, Decimal>> {
        let total =
            calc_decimal(self.item_price, self.item_qty, self.user_count)?;

        Ok(self.users.iter().map(|user| (user.id, total)).collect())
    }
}
