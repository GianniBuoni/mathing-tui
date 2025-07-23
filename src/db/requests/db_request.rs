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
    /// Returns an array of DbRequests related to fetching all table data
    /// with offsets set to 0. Should only be called during app initialization.
    pub fn init() -> Vec<Self> {
        let payloads = [
            DbPayload::ItemParams(ItemParams::default()),
            DbPayload::UserParams(UserParams::default()),
            DbPayload::ReceiptParams(JoinedReceiptParams::default()),
        ];
        let mut init = payloads
            .iter()
            .map(|payload| {
                Self::new()
                    .with_req_type(RequestType::GetAll)
                    .with_payload(payload.clone())
            })
            .collect::<Vec<Self>>();
        let mut counts = payloads
            .into_iter()
            .map(|payload| {
                Self::new()
                    .with_req_type(RequestType::Count)
                    .with_payload(payload)
            })
            .collect::<Vec<Self>>();
        init.append(&mut counts);

        init
    }
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
    /// Takes a request and destructures it into a tuple containing only
    /// the data the app needs to determine if a request has any
    /// related requests that should be sent with the initial request.
    pub fn try_descruct(
        &self,
    ) -> Result<(AppArm, RequestType, Option<Rc<str>>)> {
        let app_arm = TryInto::<AppArm>::try_into(&self.payload)?;
        let search_term = self.payload.get_search_term();
        Ok((app_arm, self.req_type, search_term))
    }
}
