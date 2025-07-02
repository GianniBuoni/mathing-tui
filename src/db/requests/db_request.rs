use super::*;

#[derive(Debug, Default, PartialEq)]
pub struct DbRequest {
    pub req_type: RequestType,
    pub payload: DbPayload,
}

impl DbRequest {
    /// Returns an array of DbRequests related to fetching all table data
    /// with offsets of 0.
    pub fn init() -> Vec<Self> {
        [
            DbPayload::ItemParams(ItemParams::default()),
            DbPayload::UserParams(UserParams::default()),
            DbPayload::ReceiptParams(JoinedReceiptParams::default()),
        ]
        .into_iter()
        .map(|payload| {
            let mut req = Self::new();
            req.with_req_type(RequestType::GetAll).with_payload(payload);
            req
        })
        .collect()
    }
    // TODO: make refresh offsets and limits configurable
    /// Returns a pre-built DbRequest for refetching StoreTotals and table data.
    /// This is Vec with a Refresh Requests and the three init requests.
    pub fn refresh() -> Vec<Self> {
        let mut refresh = Self::new();
        refresh
            .with_req_type(RequestType::Refresh)
            .with_payload(DbPayload::StoreTotal);

        let mut requests = Vec::with_capacity(4);
        requests.push(refresh);
        requests.append(&mut Self::init());

        requests
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_req_type(&mut self, req_type: RequestType) -> &mut Self {
        self.req_type = req_type;
        self
    }
    pub fn with_payload(&mut self, payload: DbPayload) -> &mut Self {
        self.payload = payload;
        self
    }
}
