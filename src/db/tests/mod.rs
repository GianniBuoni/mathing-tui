use super::connection::get_db;
use super::*;

use std::time::Duration;

use tokio::time::{Instant, sleep_until};

mod constants;
mod item_params;
mod receipts_params;
mod receipts_users_params;
mod user_params;

use constants::*;

#[sqlx::test]
async fn test_db_conn() {
    let conn = get_db().await;
    assert!(conn.is_ok());
}
