use super::*;
use query_builder::*;

mod query_builder;
mod request;

impl JoinedReceiptParams {
    pub fn builder() -> JoinParamsBuilder {
        JoinParamsBuilder::default()
    }
    fn with_r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct JoinParamsBuilder {
    pub users: Rc<RefCell<Vec<i64>>>,
    pub r_id: ParamOption<i64>,
    pub item_id: ParamOption<i64>,
    pub item_qty: ParamOption<i64>,
}

impl JoinParamsBuilder {
    pub fn with_r_id(&mut self, r_id: ParamOption<i64>) -> &mut Self {
        self.r_id = r_id;
        self
    }
    pub fn with_item_id(&mut self, item_id: ParamOption<i64>) -> &mut Self {
        self.item_id = item_id;
        self
    }
    pub fn with_item_qty(&mut self, item_qty: ParamOption<i64>) -> &mut Self {
        self.item_qty = item_qty;
        self
    }
    pub fn with_users(&mut self, users: Rc<RefCell<Vec<i64>>>) -> &mut Self {
        self.users = users;
        self
    }
    pub fn with_user(&mut self, u_id: i64) -> &mut Self {
        {
            let mut users = self.users.borrow_mut();
            users.push(u_id);
        }
        self
    }
    pub fn build(&self) -> JoinedReceiptParams {
        let users = self.users.clone();
        let users = users.borrow().to_owned();

        JoinedReceiptParams {
            users,
            r_id: self.r_id.unwrap(),
            item_id: self.item_id.unwrap(),
            item_qty: self.item_qty.unwrap(),
            ..Default::default()
        }
    }
}
