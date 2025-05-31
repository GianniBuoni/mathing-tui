use anyhow::Ok;

use std::rc::Rc;

use super::*;
use crate::prelude::*;

fn test_items<'db>() -> Rc<[ItemParams<'db>]> {
    TEST_ITEMS
        .iter()
        .map(|(name, price, _)| {
            ItemParams::new().item_name(*name).item_price(*price)
        })
        .collect()
}

async fn init_test(conn: &SqlitePool) -> Result<Vec<StoreItem>> {
    try_join_all(
        test_items()
            .iter()
            .map(async |params| Ok::<StoreItem>(params.post(conn).await?)),
    )
    .await
}

#[sqlx::test]
async fn test_get_items(conn: SqlitePool) -> Result<()> {
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
async fn test_add_items(conn: SqlitePool) -> Result<()> {
    try_join_all(test_items().into_iter().zip(TEST_ITEMS.into_iter()).map(
        async |(params, (want_name, want_price, _))| {
            Ok::<()>({
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
        Ok::<()>({
            let param = ItemParams::new().item_id(want.id);
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
    let param = ItemParams::new().item_id(originals.get(0).unwrap().id);

    param.delete(&conn).await?;

    let finals = get_store_items(&conn)
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
            let mut param = ItemParams::new().item_id(original.id);

            if let Some(name) = new_name {
                param = param.item_name(name);
            }

            if let Some(price) = new_price {
                param = param.item_price(price);
            }
            param
        })
        .collect::<Vec<ItemParams>>();

    let want = [
        ("Pretzel", 6.99, "Test update name and price"),
        ("Salmon", 9.49, "Test update name only"),
        ("Chips and Dip", 100., "Test update price only"),
    ];

    let got = try_join_all(update_params.into_iter().map(async |param| {
        Ok::<StoreItem>({
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
async fn test_blank_item_update(conn: SqlitePool) -> Result<()> {
    let originals = init_test(&conn).await?;
    let params = ItemParams::new().item_id(originals.get(0).unwrap().id);

    match params.update(&conn).await {
        std::result::Result::Ok(_) => {
            panic!("UPDATE suceeded, but an error was expected.")
        }
        Err(e) => {
            assert_eq!(
                "Malformed params: required field \"item name, item price\" is missing.",
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
                "Malformed params: required field \"item name\" is missing.",
                e.to_string(),
                "Test malformed post param"
            )
        }
    }

    Ok(())
}
