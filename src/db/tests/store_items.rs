use super::*;

async fn init_test(
    conn: &SqlitePool,
) -> Result<Vec<StoreItem>, Box<dyn Error>> {
    let rows =
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, _)| {
            Ok::<StoreItem, Box<dyn Error>>(
                add_store_item(conn, name, price).await?,
            )
        }))
        .await?;

    Ok(rows)
}

#[sqlx::test]
async fn test_add_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    assert!(
        init_test(&conn).await.is_ok(),
        "Test if items were added to db"
    );
    Ok(())
}

#[sqlx::test]
async fn test_get_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let unordered = init_test(&conn).await?;
    let ordered = get_store_items(&conn).await?;

    assert_eq!(
        ordered.len(),
        unordered.len(),
        "Test row count and amount items added match."
    );
    assert_eq!(
        "Chips and Dip", ordered[0].name,
        "Test if returned items are ordered alphabetically."
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_item_single(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        Ok::<(), Box<dyn Error>>({
            let got = get_store_item_single(&conn, want.id).await?;
            assert_eq!(want, got, "Teet if id returns expected item");
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let added_items = init_test(&conn).await?;
    delete_store_item(&conn, added_items[0].id).await?;

    let items = get_store_items(&conn)
        .await?
        .into_iter()
        .map(|item| item.name)
        .collect::<Vec<String>>();

    assert_ne!(added_items.len(), items.len(), "Test if item was deleted");
    assert!(!items.contains(&"PB Pretzel".to_string()));

    Ok(())
}

#[sqlx::test]
async fn test_update_items(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let update_params = [
        (Some("Pretzel"), Some(6.99)),
        (Some("Salmon"), None),
        (None, Some(100.)),
    ];
    let want = [
        ("Pretzel", 6.99, "Test update name and price"),
        ("Salmon", 9.49, "Test update name only"),
        ("Chips and Dip", 100., "Test update price only"),
    ];
    let got = try_join_all(
        init_test(&conn).await?.into_iter().zip(update_params).map(
            async |(item, (name, price)): (
                StoreItem,
                (Option<&str>, Option<f64>),
            )| {
                Ok::<StoreItem, Box<dyn Error>>({
                    sleep_until(Instant::now() + Duration::from_secs(1)).await;
                    update_store_item(&conn, item.id, name, price).await?;
                    get_store_item_single(&conn, item.id).await?
                })
            },
        ),
    )
    .await?;

    want.into_iter()
        .zip(got)
        .for_each(|((name, price, desc), got)| {
            assert_eq!(name, got.name, "{desc}");
            assert_eq!(price, got.price, "{desc}");
            assert_ne!(
                got.created_at, got.updated_at,
                "Test if updated_at was updated"
            );
        });

    Ok(())
}

#[sqlx::test]
async fn test_blank_item_update(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    for (name, price, _) in TEST_ITEMS {
        let new_item = add_store_item(&conn, name, price).await?;

        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        update_store_item(&conn, new_item.id, None, None).await?;
        let updated_item = get_store_item_single(&conn, new_item.id).await?;
        assert_eq!(
            updated_item.created_at, updated_item.updated_at,
            "None updates should've returned early"
        );
    }

    Ok(())
}
