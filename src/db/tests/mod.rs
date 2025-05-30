use super::connection::get_db;
use super::*;

use std::time::Duration;

use tokio::time::{Instant, sleep_until};

mod constants;
mod item_params;
mod store_receipts;
mod store_receipts_users;
mod store_receits_users_init;
mod store_users;

use constants::*;

#[sqlx::test]
async fn test_db_conn() {
    let conn = get_db().await;
    assert!(conn.is_ok());
}
