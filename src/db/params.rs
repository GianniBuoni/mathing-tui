pub mod prelude {
    pub use super::{ItemParams, ReceiptParams};
}

pub struct ItemParams {
    pub item_name: String,
    pub item_price: f64,
}

pub struct ReceiptParams {
    pub item_id: i64,
    pub item_qty: i64,
    pub users: Vec<i64>,
}
