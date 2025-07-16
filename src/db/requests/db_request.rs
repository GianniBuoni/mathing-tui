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
        let mut init = [
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
        .collect::<Vec<Self>>();
        init.append(&mut Self::counts());

        init
    }
    pub fn counts() -> Vec<Self> {
        [
            DbPayload::ItemParams(ItemParams::default()),
            DbPayload::UserParams(UserParams::default()),
            DbPayload::ReceiptParams(JoinedReceiptParams::default()),
        ]
        .into_iter()
        .map(|payload| {
            let mut req = Self::new();
            req.with_req_type(RequestType::Count).with_payload(payload);
            req
        })
        .collect()
    }
    /// Refreshes StoreTotals; should be accompanied by table refreshes as well.
    pub fn refresh() -> Self {
        let mut refresh = Self::new();
        refresh.with_payload(DbPayload::StoreTotal);
        refresh
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
