use super::*;

async fn init_test(
    conn: &SqlitePool,
) -> Result<Vec<StoreReceipt>, Box<dyn Error>> {
    let mut rows = vec![];

    for (name, price, qty) in TEST_ITEMS.into_iter() {
        let item = add_store_item(conn, name, price).await?;
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        let r = add_store_receipt(conn, item.id, qty).await?;
        rows.push(r);
    }

    Ok(rows)
}

#[sqlx::test]
async fn test_add_receipts(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    assert!(
        init_test(&conn).await.is_ok(),
        "Test if receipts are added."
    );
    Ok(())
}

#[sqlx::test]
async fn test_get_receipts(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let want = init_test(&conn).await?;
    let got = get_store_receipts(&conn, 0).await?;

    assert_eq!(
        want.len(),
        got.len(),
        "Test added receipt match len of items"
    );

    want.into_iter().zip(got).for_each(|(want, got)| {
        assert_eq!(
            want, got,
            "Test if returned receipt qty matches expected order."
        )
    });

    Ok(())
}

#[sqlx::test]
async fn test_cascade_del(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let init = init_test(&conn).await?;

    delete_store_item(&conn, 1).await?;
    let receipts = get_store_receipts(&conn, 0).await?;
    assert_ne!(
        receipts.len(),
        init.len(),
        "Deleted items should affect receipt table"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_single_receipt(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    try_join_all(init_test(&conn).await?.into_iter().map(async |want| {
        Ok::<(), Box<dyn Error>>({
            let got = get_store_receipt_single(&conn, want.id).await?;
            assert_eq!(want, got, "Test if id matches expected receipt");
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_delete_receipt(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let receipts = init_test(&conn).await?;
    delete_store_receipt_single(&conn, receipts[0].id).await?;

    let got =
        try_join_all(get_store_receipts(&conn, 0).await?.into_iter().map(
            async |r| {
                Ok::<String, Box<dyn Error>>({
                    get_store_item_single(&conn, r.item_id).await?.name
                })
            },
        ))
        .await?;

    assert_ne!(
        receipts.len(),
        got.len(),
        "Test if receipt row was deleted."
    );
    assert!(
        !got.contains(&"PB Pretzel".to_string()),
        "Test if expected receipt row was deleted"
    );

    Ok(())
}

#[sqlx::test]
async fn test_delete_store_receipts(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    delete_store_receipts(&conn).await?;

    assert_eq!(
        get_store_receipts(&conn, 0).await?.len(),
        0,
        "Single function call should have deleted all table rowss."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_receipt(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    let update_params =
        [(Some(10 as i64), "Change qty"), (None, "Change nothing")];

    try_join_all(init_test(&conn).await?.into_iter().zip(update_params).map(
        async |(receipt, (qty, desc))| {
            Ok::<(), Box<dyn Error>>({
                sleep_until(Instant::now() + Duration::from_secs(1)).await;
                update_store_receipt(&conn, receipt.id, qty).await?;
                let updated_receipt =
                    get_store_receipt_single(&conn, receipt.id).await?;

                match qty {
                    Some(_) => {
                        assert_ne!(receipt.item_qty, updated_receipt.item_qty);
                        assert_ne!(
                            updated_receipt.created_at,
                            updated_receipt.updated_at,
                            "{desc}"
                        );
                    }
                    None => {
                        assert_eq!(receipt.item_qty, updated_receipt.item_qty);
                        assert_eq!(
                            updated_receipt.created_at,
                            updated_receipt.updated_at,
                            "{desc}"
                        );
                    }
                }
            })
        },
    ))
    .await?;

    let first_item = &get_store_receipts(&conn, 0).await?[0];
    let first_item = get_store_item_single(&conn, first_item.item_id).await?;
    assert_eq!(
        first_item.name, "Slamin' Salmon",
        "Test if returned receipt list get reordered"
    );

    Ok(())
}
