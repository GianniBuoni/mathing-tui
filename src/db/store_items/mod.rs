pub mod prelude {
    pub use super::StoreItem;
    pub use super::queries::*;
}

mod queries;
#[cfg(test)]
mod tests;

pub struct StoreItem {
    created_at: i64,
    updated_at: i64,
    name: String,
    id: i64,
    price: f64,
}

impl StoreItem {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn created_at(&self) -> i64 {
        self.created_at
    }
    pub fn updated_at(&self) -> i64 {
        self.updated_at
    }
}
