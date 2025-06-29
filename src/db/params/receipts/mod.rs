use super::*;

mod request;

impl ReceiptParams {
    pub fn new() -> Self {
        ReceiptParams::default()
    }
    pub fn r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }
    pub fn item_id(mut self, item_id: i64) -> Self {
        self.item_id = Some(item_id);
        self
    }
    pub fn item_qty(mut self, item_qty: i64) -> Self {
        self.item_qty = Some(item_qty);
        self
    }
}

impl From<&JoinedReceiptParams> for ReceiptParams {
    fn from(value: &JoinedReceiptParams) -> Self {
        ReceiptParams {
            r_id: value.r_id,
            item_id: value.item_id,
            item_qty: value.item_qty,
        }
    }
}
