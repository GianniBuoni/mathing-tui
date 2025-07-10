use common::prelude::*;

mod common;

#[sqlx::test]
async fn test_add_items(conn: SqlitePool) {
    let test_cases = [("Apple", 0.29), ("Banana", 0.26), ("Grapes", 6.49)];

    let got =
        try_join_all(test_cases.into_iter().map(async |(name, price)| {
            ItemParams::builder()
                .with_item_name(ParamOption::new().map_value(name).to_owned())
                .with_item_price(ParamOption::new().map_value(price).to_owned())
                .build()
                .post(&conn)
                .await
        }))
        .await;

    assert!(got.is_ok(), "Test ItemParam post method.")
}

#[sqlx::test]
async fn test_get_items(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    // test configured limits and offset
    let test_cases = [
        (
            ItemParams::default(),
            3 as usize,
            "Chips and Dip",
            "GetAll w/ no params.",
        ),
        (
            ItemParams::default().with_limit(1),
            1,
            "Chips and Dip",
            "GetAll w/ custom limit.",
        ),
        (
            ItemParams::builder()
                .with_search(ParamOption::new().map_value("salmon").to_owned())
                .build(),
            1,
            "Slamin' Salmon",
            "GetAll w/ search param.",
        ),
    ];

    try_join_all({
        test_cases.into_iter().map(
            async |(param, want_index, want_name, desc)| {
                let got = param.get_all(&conn).await?;
                assert_eq!(
                    want_index,
                    got.len(),
                    "Test if {desc} row count matches expected"
                );
                assert_eq!(
                    want_name,
                    got.first().unwrap().name,
                    "Test if returned items are ordered alphabetically."
                );
                Aok(())
            },
        )
    })
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_get_item_single(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    try_join_all(MOCK_ITEMS.into_iter().map(async |(id, name, price)| {
        let got = ItemParams::builder()
            .with_item_id(ParamOption::new().map_value(id).to_owned())
            .build()
            .get(&conn)
            .await?;

        assert_eq!(name, got.name, "Test get method matches expected name");
        assert_eq!(price, got.price, "Test get method matches expected price");
        Aok(())
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_item(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    ItemParams::builder()
        .with_item_id(ParamOption::new().map_value(1).clone())
        .build()
        .delete(&conn)
        .await?;

    let got = ItemParams::default()
        .with_offset(0)
        .get_all(&conn)
        .await?
        .into_iter()
        .map(|item| item.name)
        .collect::<Vec<String>>();

    assert_ne!(MOCK_ITEMS.len(), got.len(), "Test if item was deleted.");
    assert!(
        !got.contains(&"PB Pretzel".to_string()),
        "Test if correct item was deleted."
    );
    Ok(())
}

#[sqlx::test]
async fn test_update_item(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let update_params = [
        ItemParams::builder()
            .with_item_id(ParamOption::new().map_value(1).to_owned())
            .with_item_name(ParamOption::new().map_value("Pretzel").to_owned())
            .with_item_price(ParamOption::new().map_value(6.99).to_owned())
            .build(),
        ItemParams::builder()
            .with_item_id(ParamOption::new().map_value(2).to_owned())
            .with_item_name(ParamOption::new().map_value("Salmon").to_owned())
            .build(),
        ItemParams::builder()
            .with_item_id(ParamOption::new().map_value(3).to_owned())
            .with_item_price(ParamOption::new().map_value(100.).to_owned())
            .build(),
    ];

    let got = try_join_all(
        update_params
            .into_iter()
            .map(async |param| param.update(&conn).await),
    )
    .await?;

    let want = [
        ("Pretzel", 6.99, "Test update name and price"),
        ("Salmon", 9.49, "Test update name only"),
        ("Chips and Dip", 100., "Test update price only"),
    ];

    want.into_iter().zip(got).for_each(
        |((want_name, want_price, desc), got)| {
            assert_eq!(want_name, got.name, "{desc}");
            assert_eq!(want_price, got.price, "{desc}");
        },
    );
    Ok(())
}

#[sqlx::test]
async fn test_item_count(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let got = ItemParams::default().count(&conn).await?;
    assert_eq!(3, got, "Test if item count matches expected.");

    Ok(())
}
