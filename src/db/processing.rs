use super::{prelude::Request, *};

impl StoreJoinRaw {
    pub async fn as_join_row(&self, conn: &SqlitePool) -> Result<StoreJoinRow> {
        let users = try_join_all(self.user_ids.split(",").map(async |s| {
            Ok::<StoreUser, Error>({
                let id = s.parse::<i64>()?;
                UserParams::new().user_id(id).get(conn).await?
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

impl StoreJoinRow {
    pub fn calc(&self) -> HashMap<i64, Decimal> {
        let mut out = HashMap::new();

        if let Some(total) = Decimal::from_f64(
            self.item_price * self.item_qty as f64 / self.user_count as f64,
        ) {
            self.users.iter().for_each(|u| {
                out.insert(u.id, total.round_dp(2));
            });
        };

        out
    }
}

impl StoreTotal {
    pub fn add(&mut self, other: HashMap<i64, Decimal>) {
        other.into_iter().for_each(|(key, val)| {
            self.0.entry(key).and_modify(|e| *e += val).or_insert(val);
        });
    }
}
