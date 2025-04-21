use std::{env, error::Error};

use sqlx::SqlitePool;

use crate::prelude::*;

#[sqlx::test]
async fn test_add_and_get_items(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    let test_items = [
        ("PB Pretzel", 4.99),
        ("Slamin' Salmon", 9.49),
        ("Chips and Dip", 5.55),
    ];

    for item in test_items {
        let test_item = add_items(&conn, item.0, item.1).await?;
        assert_eq!(item.0, test_item.name(), "Test new item's name match.");
        assert_eq!(item.1, test_item.price(), "Test new item's price match.");
    }

    let test_fetch = get_items(&conn).await?;
    assert_eq!(
        test_items.len(),
        test_fetch.len(),
        "Test row count and amount items added match."
    );
    assert_eq!(
        "Chips and Dip",
        test_fetch[0].name(),
        "Test db returning in alphabetical order."
    );

    Ok(())
}

#[sqlx::test]
async fn test_db_conn() -> Result<(), Box<dyn Error>> {
    if let Ok(is_production) = env::var("PLATFORM") {
        if is_production == "production" {
            let msg =
                "
                You're in production mode! Run this test only in development mode
                ";
            return Err(msg.into());
        }
    }
    let conn = get_db().await?;

    let test_items = [
        ("PB Pretzel", 4.99),
        ("Slamin' Salmon", 9.49),
        ("Chips and Dip", 5.55),
    ];

    let original_len = get_items(conn).await?.len();

    for item in test_items {
        let new_item = add_items(conn, item.0, item.1).await?;
        delete_items(conn, new_item.id()).await?;
    }

    let new_len = get_items(conn).await?.len();
    assert_eq!(original_len, new_len, "Test adding removing rows.");

    Ok(())
}
