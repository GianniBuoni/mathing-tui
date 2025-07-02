use std::time::Duration;

use tokio::time::{Instant, sleep_until};

use super::*;

mod item_params;
mod join_params;
mod receipts_params;
mod receipts_users_params;
mod total_params;
mod user_params;

pub const TEST_USERS: [&str; 3] = ["Thing", "Noodle", "Jon"];

pub const TEST_ITEMS: [(&str, f64, i64); 3] = [
    ("PB Pretzel", 4.99, 2),
    ("Slamin' Salmon", 9.49, 1),
    ("Chips and Dip", 5.55, 3),
];

impl StoreJoinPrices {
    fn mock() -> Vec<Self> {
        let r_1 = Self {
            user_ids: "3".into(),
            user_count: 1,
            item_price: 4.99,
            item_qty: 2,
        };
        let r_2 = Self {
            user_ids: "2".into(),
            user_count: 1,
            item_price: 9.49,
            item_qty: 1,
        };
        let r_3 = Self {
            user_ids: "2,3".into(),
            user_count: 2,
            item_price: 5.55,
            item_qty: 3,
        };

        vec![r_1, r_2, r_3]
    }
}

impl StoreTotal {
    fn mock() -> Self {
        [(3, dec!(18.30)), (2 as i64, dec!(17.81))]
            .into_iter()
            .fold(StoreTotal::default(), |mut acc, next| {
                acc.add(HashMap::from([next]));
                acc
            })
    }
}

async fn init_users(conn: &SqlitePool) -> Result<Vec<StoreUser>> {
    let mut res = vec![];
    for name in TEST_USERS {
        let user = UserParams::builder()
            .with_user_name(ParamOption::new().map_value(name).clone())
            .build()
            .post(conn)
            .await?;
        res.push(user);
    }
    Ok(res)
}

async fn intit_items(conn: &SqlitePool) -> Result<Vec<(StoreItem, i64)>> {
    let mut res = vec![];
    for (name, price, qty) in TEST_ITEMS {
        let item = ItemParams::builder()
            .with_item_name(ParamOption::new().map_value(name).clone())
            .with_item_price(ParamOption::new().map_value(price).clone())
            .build()
            .post(conn)
            .await?;
        res.push((item, qty));
    }
    Ok(res)
}

async fn init_reciepts(conn: &SqlitePool) -> Result<Vec<StoreReceipt>> {
    anyhow::Ok(
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, qty)| {
            anyhow::Ok::<StoreReceipt>({
                let item = ItemParams::builder()
                    .with_item_name(ParamOption::new().map_value(name).clone())
                    .with_item_price(
                        ParamOption::new().map_value(price).clone(),
                    )
                    .build()
                    .post(&conn)
                    .await?;

                let mut tx = conn.begin().await?;
                let receipts = ReceiptParams::new()
                    .item_id(item.id)
                    .item_qty(qty)
                    .post(&mut *tx)
                    .await?;
                tx.commit().await?;

                receipts
            })
        }))
        .await?,
    )
}

pub async fn init_join_rows(conn: &SqlitePool) -> Result<Vec<StoreJoinRow>> {
    let users = init_users(conn).await?;
    let items = intit_items(conn).await?;
    let mut res = vec![];

    for (index, r) in items.into_iter().enumerate() {
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        let mut param = JoinedReceiptParams::builder();
        param
            .with_item_id(ParamOption::new().map_value(r.0.id).clone())
            .with_item_qty(ParamOption::new().map_value(r.1).clone())
            .with_offset(0);

        match index {
            0 => {
                // add Jon to PB Pretzel
                param.with_user(users.get(2).unwrap().id);
            }
            1 => {
                // add Noodle to Salmon
                param.with_user(users.get(1).unwrap().id);
            }
            2 => {
                // add Noodle and Jon to Chips and Dip
                param
                    .with_user(users.get(1).unwrap().id)
                    .with_user(users.get(2).unwrap().id);
            }
            _ => {}
        }
        res.push(param.build().post(conn).await?);
    }
    Ok(res)
}
