use super::*;

impl DbResponse {
    pub fn new(req_type: RequestType) -> Self {
        Self {
            req_type,
            payload: DbPayload::default(),
            error: None,
        }
    }
    pub fn payload(mut self, payload: DbPayload) -> Self {
        self.payload = payload;
        self
    }
    pub fn error(mut self, e: impl ToString) -> Self {
        self.error = Some(e.to_string());
        self
    }
}
