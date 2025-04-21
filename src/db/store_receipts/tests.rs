use std::error::Error;

use sqlx::SqlitePool;

use crate::prelude::*;

const TEST_ITEMS: [(&str, f64, i64); 3] = [
    ("Pizza", 10., 1),
    ("Pastry Pups", 4.49, 2),
    ("Tacquitos", 3.49, 4),
];

async fn init_test(conn: &SqlitePool) -> Result<(), Box<dyn Error>> {
    for (name, price, qty) in TEST_ITEMS {
        let new_item = add_store_item(conn, name, price).await?;
        let new_receipt = add_store_receipt(&conn, new_item.id(), qty).await;
        assert!(new_receipt.is_ok(), "Test successful reciept add");
    }
    Ok(())
}

#[sqlx::test]
async fn test_add_receipts(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_get_receipts(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let receipts = get_store_receipts(&conn).await?;
    assert_eq!(
        TEST_ITEMS.len(),
        receipts.len(),
        "Test added receipt match len of items"
    );

    Ok(())
}

#[sqlx::test]
async fn test_cascade_del(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    delete_store_item(&conn, 1).await?;
    let receipts = get_store_receipts(&conn).await?;
    assert_eq!(
        receipts.len(),
        2,
        "Deleted items should have deleted receipt as well"
    );

    Ok(())
}
