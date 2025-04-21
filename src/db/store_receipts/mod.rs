pub mod prelude {
    pub use super::StoreReceipt;
    pub use super::queries::*;
}

mod queries;
#[cfg(test)]
mod tests;

pub struct StoreReceipt {
    id: i64,
    created_at: i64,
    updated_at: i64,
    item_id: i64,
    item_qty: i64,
}

impl StoreReceipt {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn created_at(&self) -> i64 {
        self.created_at
    }
    pub fn updated_at(&self) -> i64 {
        self.updated_at
    }
    pub fn item_id(&self) -> i64 {
        self.item_id
    }
    pub fn item_qty(&self) -> i64 {
        self.item_qty
    }
}
