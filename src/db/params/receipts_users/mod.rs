use super::*;

mod request;

impl ReceiptsUsersParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_r_id(mut self, r_id: i64) -> Self {
        self.r_id = Some(r_id);
        self
    }
    pub fn with_u_id(mut self, u_id: i64) -> Self {
        self.u_id = Some(u_id);
        self
    }
}
