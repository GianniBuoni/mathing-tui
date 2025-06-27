use super::{prelude::Request, *};

impl StoreJoinRaw {
    pub async fn try_join_row(self, conn: &SqlitePool) -> Result<StoreJoinRow> {
        let users = try_join_all(self.user_ids.split(",").map(async |s| {
            Aok::<StoreUser>({
                let id = s.parse::<i64>()?;
                UserParams::new().user_id(id).get(conn).await?
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
impl StoreJoinRow {
    pub fn try_calc(&self) -> Result<HashMap<i64, Decimal>> {
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
impl DbTable {
    pub fn try_get_item(&self) -> Result<&StoreItem, AppError> {
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
    pub fn try_get_user(&self) -> Result<&StoreUser, AppError> {
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
    pub fn try_get_receipt(&self) -> Result<&StoreJoinRow, AppError> {
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
