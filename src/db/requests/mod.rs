use super::*;

pub mod prelude {
    pub use super::Request;
    pub use super::db_request::DbRequest;
    pub use super::handle_requests::handle_requests;
    pub use super::req_type::RequestType;
}

mod db_request;
mod handle_requests;
mod req_type;

pub trait Request {
    type Output: Into<DbPayload>;
    type Outputs: Into<DbPayload>;

    // required methods
    fn check_id(&self, req_type: RequestType) -> Result<i64, RequestError>;
    fn get_app_arm(&self) -> AppArm;
    fn get_all(
        &self,
        conn: &SqlitePool,
    ) -> impl Future<Output = Result<Self::Outputs>>;
    fn get(
        &self,
        conn: &SqlitePool,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn post(
        &self,
        conn: &SqlitePool,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn update(
        &self,
        conn: &SqlitePool,
    ) -> impl Future<Output = Result<Self::Output>>;
    fn delete(&self, conn: &SqlitePool) -> impl Future<Output = Result<u64>>;

    // optional methods
    fn count(&self, conn: &SqlitePool) -> impl Future<Output = Result<i64>> {
        async {
            let _ = conn;
            Err(RequestError::unhandled("request type", "Count").into())
        }
    }
    fn reset(&self, conn: &SqlitePool) -> impl Future<Output = Result<u64>> {
        async {
            let _ = conn;
            Err(RequestError::unhandled("request type", "Reset").into())
        }
    }
    fn make_request(
        &self,
        conn: &SqlitePool,
        req_type: RequestType,
    ) -> impl Future<Output = Result<DbPayload>> {
        async move {
            let res = match req_type {
                RequestType::GetAll => self.get_all(conn).await?.into(),
                RequestType::Get => self.get(conn).await?.into(),
                RequestType::Post => self.post(conn).await?.into(),
                RequestType::Update => self.update(conn).await?.into(),
                RequestType::Delete => self
                    .delete(conn)
                    .await
                    .map(|f| DbPayload::AffectedRows(self.get_app_arm(), f))?,
                RequestType::Reset => self
                    .reset(conn)
                    .await
                    .map(|f| DbPayload::AffectedRows(self.get_app_arm(), f))?,
                RequestType::Count => self
                    .count(conn)
                    .await
                    .map(|count| DbPayload::Count(self.get_app_arm(), count))?,
                _ => {
                    return Err(RequestError::unhandled(
                        "request type",
                        format!("{req_type:?}"),
                    )
                    .into());
                }
            };

            Ok(res)
        }
    }
}
