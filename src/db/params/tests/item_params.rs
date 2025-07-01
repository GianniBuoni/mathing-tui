use std::rc::Rc;

use super::*;

fn test_items() -> Rc<[ItemParams]> {
    TEST_ITEMS
        .iter()
        .map(|(name, price, _)| {
            ItemParams::builder()
                .with_item_name(ParamOption::new().map_value(*name).clone())
                .with_item_price(ParamOption::new().map_value(*price).clone())
                .build()
        })
        .collect()
}

async fn init_test(conn: &SqlitePool) -> Result<Vec<StoreItem>> {
    try_join_all(
        test_items()
            .iter()
            .map(async |params| Aok::<StoreItem>(params.post(conn).await?)),
    )
    .await
}

#[sqlx::test]
async fn test_get_items(conn: SqlitePool) -> Result<()> {
    let _ = init_test(&conn).await?;

    // test configured limits and offset
    let test_cases = [
        (
            ItemParams::builder().build(),
            3 as usize,
            "Chips and Dip",
            "GetAll w/ no params.",
        ),
        (
            ItemParams::builder().with_limit(1).build(),
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
                Aok({
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
                })
            },
        )
    })
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_add_items(conn: SqlitePool) -> Result<()> {
    try_join_all(test_items().into_iter().zip(TEST_ITEMS.into_iter()).map(
        async |(params, (want_name, want_price, _))| {
            Aok({
                let got = params.post(&conn).await?;

                assert_eq!(
                    want_name.to_string(),
                    got.name,
                    "Test added item matches expected output."
                );
                assert_eq!(
                    want_price.to_string(),
                    got.price.to_string(),
                    "Test added item price matches expected output."
                );
            })
        },
    ))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_get_item_single(conn: SqlitePool) -> Result<()> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        Aok({
            let param = ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(want.id).clone())
                .build();
            let got = param.get(&conn).await?;
            assert_eq!(want.name, got.name);
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_item(conn: SqlitePool) -> Result<()> {
    let originals = init_test(&conn).await?;
    let param = ItemParams::builder()
        .with_item_id(
            ParamOption::new()
                .map_value(originals.get(0).unwrap().id)
                .clone(),
        )
        .build();

    param.delete(&conn).await?;

    let finals = ItemParams::builder()
        .with_offset(0)
        .build()
        .get_all(&conn)
        .await?
        .into_iter()
        .map(|item| item.name)
        .collect::<Vec<String>>();

    assert_ne!(originals.len(), finals.len(), "Test if item was deleted.");
    assert!(
        !finals.contains(&"PB Pretzel".to_string()),
        "Test if correct item was deleted."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_item(conn: SqlitePool) -> Result<()> {
    let originals = init_test(&conn).await?;

    let update_params = [
        (Some("Pretzel"), Some(6.99)),
        (Some("Salmon"), None),
        (None, Some(100.)),
    ];
    let update_params = originals
        .iter()
        .zip(update_params.into_iter())
        .map(|(original, (new_name, new_price))| {
            let mut param = ItemParams::builder();
            param.with_item_id(
                ParamOption::new().map_value(original.id).clone(),
            );

            if let Some(name) = new_name {
                param
                    .with_item_name(ParamOption::new().map_value(name).clone());
            }

            if let Some(price) = new_price {
                param.with_item_price(
                    ParamOption::new().map_value(price).clone(),
                );
            }
            param.build()
        })
        .collect::<Vec<ItemParams>>();

    let want = [
        ("Pretzel", 6.99, "Test update name and price"),
        ("Salmon", 9.49, "Test update name only"),
        ("Chips and Dip", 100., "Test update price only"),
    ];

    let got = try_join_all(update_params.into_iter().map(async |param| {
        Aok::<StoreItem>({
            sleep_until(Instant::now() + Duration::from_secs(1)).await;
            let item = param.update(&conn).await?;
            item
        })
    }))
    .await?;

    want.into_iter()
        .zip(got.into_iter().zip(originals))
        .for_each(|((want_name, want_price, desc), (got, original))| {
            assert_eq!(want_name, got.name, "{desc}");
            assert_eq!(want_price, got.price, "{desc}");
            assert_ne!(
                got.updated_at, original.updated_at,
                "Test if updated feild updated."
            );
        });

    Ok(())
}

#[sqlx::test]
async fn test_item_count(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;
    let got = ItemParams::default().count(&conn).await;
    assert_eq!(3, got, "Test if item count matches expected.");

    Ok(())
}

#[sqlx::test]
async fn test_blank_item_update(conn: SqlitePool) -> Result<()> {
    let originals = init_test(&conn).await?;
    let params = ItemParams::builder()
        .with_item_id(
            ParamOption::new()
                .map_value(originals.get(0).unwrap().id)
                .clone(),
        )
        .build();

    match params.update(&conn).await {
        std::result::Result::Ok(_) => {
            panic!("UPDATE suceeded, but an error was expected.")
        }
        Err(e) => {
            assert_eq!(
                RequestError::missing_param(
                    RequestType::Update,
                    "item",
                    "item name, item price"
                )
                .to_string(),
                e.to_string(),
                "Test malformed update params."
            );
        }
    }
    match params.post(&conn).await {
        std::result::Result::Ok(_) => {
            panic!("POST suceeded, but an error was expected.")
        }
        Err(e) => {
            assert_eq!(
                RequestError::missing_param(
                    RequestType::Post,
                    "item",
                    "item name"
                )
                .to_string(),
                e.to_string(),
                "Test malformed post param"
            )
        }
    }

    Ok(())
}
