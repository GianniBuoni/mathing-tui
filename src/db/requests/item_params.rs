use super::*;

impl ItemParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn item_name(mut self, name: impl ToString) -> Self {
        self.item_name = Some(name.to_string());
        self
    }

    pub fn item_price(mut self, price: f64) -> Self {
        self.item_price = Some(price);
        self
    }

    fn check_id(&self) -> Result<i64> {
        self.item_id
            .ok_or(anyhow::Error::msg("Required field \"id\" is empty."))
    }
}
