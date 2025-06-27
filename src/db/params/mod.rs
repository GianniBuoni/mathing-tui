use super::*;

pub mod prelude {
    pub use super::{ItemParams, JoinedReceiptParams, ParamOption, UserParams};
}

pub(super) mod items;
pub(super) mod join_row;
pub(super) mod receipts;
pub(super) mod receipts_users;
#[cfg(test)]
pub(super) mod tests;
pub(super) mod users;

#[derive(Debug, Default, Clone)]
pub struct ParamOption<T>(Rc<RefCell<Option<T>>>)
where
    T: Default + Debug;

impl<T> ParamOption<T>
where
    T: Default + Debug + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    // TODO: figure out how to aviod a clone here.
    // the build method needs to operate on a reference
    // since form types own the builder type
    pub fn unwrap(&self) -> Option<T> {
        self.0.borrow().deref().clone()
    }
    pub fn map_value(&self, value: impl Into<T>) -> &Self {
        {
            *self.0.borrow_mut() = Some(value.into());
        }
        self
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserParams {
    u_id: Option<i64>,
    name: Option<String>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ItemParams {
    item_id: Option<i64>,
    item_name: Option<String>,
    item_price: Option<f64>,
    offset: Option<i64>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinedReceiptParams {
    users: Vec<i64>,
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Default)]
pub(super) struct ReceiptsUsersParams {
    r_id: Option<i64>,
    u_id: Option<i64>,
}

#[derive(Debug, Default)]
pub(super) struct ReceiptParams {
    r_id: Option<i64>,
    item_id: Option<i64>,
    item_qty: Option<i64>,
}
