use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct DbRequest {
    pub req_type: RequestType,
    pub payload: DbPayload,
}

impl DbRequest {
    /// Request to refresh store totals
    pub const STORE_TOTAL: Self = Self {
        req_type: RequestType::None,
        payload: DbPayload::StoreTotal,
    };
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_req_type(mut self, req_type: RequestType) -> Self {
        self.req_type = req_type;
        self
    }
    pub fn with_payload(mut self, payload: DbPayload) -> Self {
        self.payload = payload;
        self
    }
}
