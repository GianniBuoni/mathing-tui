use super::*;

impl DbResponse {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn req_type(mut self, req_type: RequestType) -> Self {
        self.req_type = req_type;
        self
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

impl DbRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn req_type(mut self, req_type: RequestType) -> Self {
        self.req_type = req_type;
        self
    }
    pub fn payload(mut self, payload: DbPayload) -> Self {
        self.payload = payload;
        self
    }
}
