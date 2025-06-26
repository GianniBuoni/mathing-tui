use crate::{app::AppArm, prelude::AppError};

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
    pub fn calc(&self) -> Result<HashMap<i64, Decimal>> {
        let err = "Decimal Error: Could not convert float to Decimal";
        let total = Decimal::from_f64(
            self.item_price * self.item_qty as f64 / self.user_count as f64,
        )
        .ok_or(anyhow::Error::msg(err))?;

        Ok({
            self.users
                .iter()
                .map(|user| (user.id, total.round_dp(2)))
                .collect()
        })
    }
}

impl StoreTotal {
    pub fn add(&mut self, other: HashMap<i64, Decimal>) {
        other.into_iter().for_each(|(key, val)| {
            self.0.entry(key).and_modify(|e| *e += val).or_insert(val);
        });
    }
}

impl DbTable {
    pub fn get_item(&self) -> Result<&StoreItem, AppError> {
        match self {
            DbTable::Item(i) => Ok(i),
            DbTable::User(_) => {
                Err(AppError::Mapping(AppArm::Items, AppArm::Users))
            }
            DbTable::Receipt(_) => {
                Err(AppError::Mapping(AppArm::Items, AppArm::Receipts))
            }
        }
    }
    pub fn get_user(&self) -> Result<&StoreUser, AppError> {
        match self {
            DbTable::Item(_) => {
                Err(AppError::Mapping(AppArm::Users, AppArm::Items))
            }
            DbTable::User(u) => Ok(u),
            DbTable::Receipt(_) => {
                Err(AppError::Mapping(AppArm::Users, AppArm::Receipts))
            }
        }
    }
    pub fn get_receipt(&self) -> Result<&StoreJoinRow, AppError> {
        match self {
            DbTable::Item(_) => {
                Err(AppError::Mapping(AppArm::Receipts, AppArm::Items))
            }
            DbTable::User(_) => {
                Err(AppError::Mapping(AppArm::Receipts, AppArm::Users))
            }
            DbTable::Receipt(r) => Ok(r),
        }
    }
}
