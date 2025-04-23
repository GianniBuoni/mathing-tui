use super::*;

use std::{error::Error, time::Duration};

use tokio::time::{Instant, sleep_until};

mod store_items;
mod store_receipts;
mod store_users;

#[sqlx::test]
async fn test_db_conn() {
    let conn = get_db().await;
    assert!(conn.is_ok());
}
