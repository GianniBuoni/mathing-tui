use super::*;

impl AppConfig {
    // mock some config directories!
}

#[sqlx::test]
async fn test_db_conn() {
    let conn = DbConn::try_init().await;
    assert!(conn.is_ok());
}
