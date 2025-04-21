use std::error::Error;

use sqlx::SqlitePool;

use crate::prelude::*;

const TEST_ITEMS: [(&str, f64); 3] = [
    ("PB Pretzel", 4.99),
    ("Slamin' Salmon", 9.49),
    ("Chips and Dip", 5.55),
];

#[sqlx::test]
async fn test_add_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    for item in TEST_ITEMS {
        let test_item = add_items(&conn, item.0, item.1).await?;
        assert_eq!(item.0, test_item.name(), "Test new item's name match.");
        assert_eq!(item.1, test_item.price(), "Test new item's price match.");
    }

    let test_fetch = get_items(&conn).await?;
    assert_eq!(
        TEST_ITEMS.len(),
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
async fn test_get_item_single(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let mut ids = vec![];

    for (name, price) in TEST_ITEMS {
        let new_items = add_items(&conn, name, price).await?;
        ids.push(new_items.id());
    }

    for ((name, price), id) in TEST_ITEMS.iter().zip(ids) {
        let item = get_item_single(&conn, id).await?;
        let desc = "Test get_item_single match expected id:";
        assert_eq!(item.name(), *name, "{desc} {id}.");
        assert_eq!(item.price(), *price, "{desc} {id}.");
    }

    Ok(())
}

#[sqlx::test]
async fn test_delete_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let original_len = get_items(&conn).await?.len();

    for item in TEST_ITEMS {
        let new_item = add_items(&conn, item.0, item.1).await?;
        delete_items(&conn, new_item.id()).await?;
    }

    let final_len = get_items(&conn).await?.len();
    assert_eq!(original_len, final_len, "Test adding removing rows.");

    Ok(())
}

#[sqlx::test]
async fn test_update_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let mut ids = vec![];

    for (name, price) in TEST_ITEMS {
        let new_item = add_items(&conn, name, price).await?;
        ids.push(new_item.id());
    }

    let update_params = [
        (Some("Pretzel"), Some(6.99)),
        (Some("Salmon"), None),
        (None, Some(100.)),
    ];

    for (id, (name, price)) in ids.iter().zip(update_params) {
        update_items(&conn, *id, name, price).await?;
    }

    let want = [
        ("Pretzel", 6.99, "Test update name and price"),
        ("Salmon", 9.49, "Test update name only"),
        ("Chips and Dip", 100., "Test update price only"),
    ];

    for (id, (name, price, desc)) in ids.iter().zip(want) {
        let test_item = get_item_single(&conn, *id).await?;
        assert_eq!(test_item.name(), name, "{desc}, id: {id}");
        assert_eq!(test_item.price(), price, "{desc}, id: {id}");
    }

    Ok(())
}
