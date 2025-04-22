pub mod prelude {
    pub use super::StoreUser;
    pub use super::queries::*;
}

mod queries;
#[cfg(test)]
mod tests;

pub struct StoreUser {
    id: i64,
    created_at: i64,
    updated_at: i64,
    name: String,
}

impl StoreUser {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn created_at(&self) -> i64 {
        self.created_at
    }
    pub fn updated_at(&self) -> i64 {
        self.updated_at
    }
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
