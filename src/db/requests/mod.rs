use super::*;

pub mod prelude {
    pub use super::handle_requests::handle_requests;
    pub use super::{DbRequest, Request, RequestType};
}

mod handle_requests;

pub trait Request<'e> {
    type Output;
    type Connection: SqliteExecutor<'e>;

    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError>;
    fn get(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn get_all(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Vec<Self::Output>>>;
    fn post(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn update(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn delete(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<u64>>;
}

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
            DbPayload::ItemParams(ItemParams::builder().with_offset(0).build()),
            DbPayload::UserParams(UserParams::builder().build()),
            DbPayload::ReceiptParams(
                JoinedReceiptParams::builder().with_offset(0).build(),
            ),
        ]
        .into_iter()
        .map(|payload| {
            let mut req = Self::new();
            req.with_req_type(RequestType::GetAll).with_payload(payload);
            req
        })
        .collect()
    }
    // TODO: make refresh offsets configurable
    /// Returns a pre-built DbRequest for refetching StoreTotals and table data.
    /// This is Vec with a Refresh Requests and the three init requests.
    pub fn refresh() -> Vec<Self> {
        let mut refresh = Self::new();
        refresh.with_req_type(RequestType::Refresh);

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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RequestType {
    #[default]
    None,
    GetAll,
    Get,
    Post,
    Update,
    Delete,
    Reset,
    Refresh,
}

impl Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::GetAll => write!(f, "Get all"),
            Self::Get => write!(f, "Get"),
            Self::Post => write!(f, "Post"),
            Self::Update => write!(f, "Update"),
            Self::Delete => write!(f, "Delete"),
            Self::Reset => write!(f, "Reset"),
            Self::Refresh => write!(f, "Refresh"),
        }
    }
}
