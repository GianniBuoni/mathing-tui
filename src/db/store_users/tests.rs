use std::error::Error;

use sqlx::SqlitePool;

use crate::prelude::*;

const TEST_ITEMS: [&str; 3] = ["Jon", "Thing", "Noodle"];

async fn init_test(conn: &SqlitePool) -> Result<(), Box<dyn Error>> {
    for name in TEST_ITEMS {
        add_store_user(conn, name).await?;
    }
    Ok(())
}

#[sqlx::test]
async fn test_add_user(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    Ok(())
}
