use std::{error::Error, time::Duration};

use sqlx::SqlitePool;
use tokio::time::{Instant, sleep_until};

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

    for (receipt, (_, _, qty)) in receipts.iter().zip(TEST_ITEMS) {
        assert_eq!(
            receipt.item_qty(),
            qty,
            "Test if returned receipt qty matches expected order."
        )
    }

    Ok(())
}

#[sqlx::test]
async fn test_cascade_del(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    delete_store_item(&conn, 1).await?;
    let receipts = get_store_receipts(&conn).await?;
    assert_ne!(
        receipts.len(),
        TEST_ITEMS.len(),
        "Deleted items should affect receipt table"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_single_receipt(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let rows = get_store_receipts(&conn).await?;
    for receipt in rows {
        let desc = "Test if getting receipt by id matches expected";
        let receipt_by_id =
            get_store_receipt_single(&conn, receipt.id()).await?;
        assert_eq!(receipt.item_id(), receipt_by_id.item_id(), "{desc}");
        assert_eq!(receipt.item_qty(), receipt_by_id.item_qty(), "{desc}");
    }

    Ok(())
}

#[sqlx::test]
async fn test_delete_receipt(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let receipts = get_store_receipts(&conn).await?;

    for receipt in receipts {
        delete_store_receipt(&conn, receipt.id()).await?;
    }

    let final_receipt = get_store_receipts(&conn).await?;
    let items = get_store_items(&conn).await?;

    assert_eq!(final_receipt.len(), 0, "Test if receipts are deleted.");
    assert_eq!(
        items.len(),
        TEST_ITEMS.len(),
        "Deleted receipts should not affect items table"
    );
    Ok(())
}

#[sqlx::test]
async fn test_update_receipt(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let receipts = get_store_receipts(&conn).await?;
    let update_params =
        [(Some(10 as i64), "Change qty"), (None, "Change nothing")];

    for (receipt, (qty, desc)) in receipts.iter().zip(update_params) {
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        update_store_receipt(&conn, receipt.id(), qty).await?;
        let updated_receipt =
            get_store_receipt_single(&conn, receipt.id()).await?;

        match qty {
            Some(_) => {
                assert_ne!(receipt.item_qty(), updated_receipt.item_qty());
                assert_ne!(
                    updated_receipt.created_at(),
                    updated_receipt.updated_at(),
                    "{desc}"
                );
            }
            None => {
                assert_eq!(receipt.item_qty(), updated_receipt.item_qty());
                assert_eq!(
                    updated_receipt.created_at(),
                    updated_receipt.updated_at(),
                    "{desc}"
                );
            }
        }
    }

    Ok(())
}
