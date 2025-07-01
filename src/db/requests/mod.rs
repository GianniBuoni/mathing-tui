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

pub trait Request<'e> {
    type Output;
    type Connection: SqliteExecutor<'e>;

    // required methods
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

    // optional methods
    fn count(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<i64>> {
        async {
            let _ = conn;
            todo!()
        }
    }
    fn reset(
        &self,
        conn: Self::Connection,
    ) -> impl Future<Output = Result<u64>> {
        async {
            let _ = conn;
            todo!()
        }
    }
}
