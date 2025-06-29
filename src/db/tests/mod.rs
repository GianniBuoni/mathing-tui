use super::requests::prelude::*;
use super::*;

mod handle_req_errors;
mod handle_requests;
mod test_cases;

#[sqlx::test]
async fn test_db_conn() {
    let conn = DbConn::try_get().await;
    assert!(conn.is_ok());
}
