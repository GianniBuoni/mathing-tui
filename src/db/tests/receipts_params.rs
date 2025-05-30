use super::*;
use crate::prelude::*;

async fn init_test(conn: &SqlitePool) -> Result<Vec<StoreReceipt>> {
    let items =
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, _)| {
            Ok::<StoreItem, Error>({
                let mut tx = conn.begin().await?;
                let item = ItemParams::new()
                    .item_name(name)
                    .item_price(price)
                    .post(&mut *tx)
                    .await?;
                tx.commit().await?;
                item
            })
        }))
        .await?;

    let mut r = vec![];

    for (item, (_, _, qty)) in items.into_iter().zip(TEST_ITEMS.into_iter()) {
        let _ = sleep_until(Instant::now() + Duration::from_secs(1));
        let mut tx = conn.begin().await?;
        let new_r = ReceiptParams::new()
            .item_id(item.id)
            .item_qty(qty)
            .post(&mut *tx)
            .await?;
        tx.commit().await?;
        r.push(new_r);
    }

    Ok(r)
}

#[sqlx::test]
async fn test_add_receipts(conn: SqlitePool) -> Result<()> {
    let receipts = init_test(&conn).await?;

    assert_eq!(
        TEST_ITEMS.len(),
        receipts.len(),
        "Test if receipts are added successfully"
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_receipts(conn: SqlitePool) -> Result<()> {
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
            "Test if returned receipts matches expected order."
        )
    });

    Ok(())
}

#[sqlx::test]
async fn test_get_receipt(conn: SqlitePool) -> Result<()> {
    let mut tx = conn.begin().await?;
    let want = init_test(&conn).await?;
    let want = want.get(0).unwrap();
    let got = ReceiptParams::new().r_id(want.id).get(&mut *tx).await?;

    assert_eq!(*want, got, "Test if get returns extpected row.");

    Ok(())
}

#[sqlx::test]
async fn test_cascade_del(conn: SqlitePool) -> Result<()> {
    let cmp = init_test(&conn).await?;

    let mut tx = conn.begin().await?;
    let affected_rows = ItemParams::new()
        .item_id(cmp.get(0).unwrap().item_id)
        .delete(&mut *tx)
        .await?;
    tx.commit().await?;

    if affected_rows.is_zero() {
        panic!("Test failed prematurely, no items deleted.")
    }

    let got = get_store_receipts(&conn, 0).await?;
    assert_ne!(
        cmp.len(),
        got.len(),
        "Deleted items should affect receipts table."
    );

    Ok(())
}

#[sqlx::test]
fn test_delete_receipt(conn: SqlitePool) -> Result<()> {
    let test_case = init_test(&conn).await?;
    let test_case = test_case.get(0).unwrap().id;

    let mut tx = conn.begin().await?;
    let affected_rows = ReceiptParams::new()
        .r_id(test_case)
        .delete(&mut *tx)
        .await?;
    tx.commit().await?;

    assert_eq!(1, affected_rows, "Test if row was deleted");

    let got = get_store_receipts(&conn, 0)
        .await?
        .into_iter()
        .map(|r| r.id)
        .collect::<Vec<i64>>();

    assert!(
        !got.contains(&test_case),
        "Test if expected row was deleted."
    );

    Ok(())
}

#[sqlx::test]
async fn test_update_receipt(conn: SqlitePool) -> Result<()> {
    let init = init_test(&conn).await?;
    sleep_until(Instant::now() + Duration::from_secs(1)).await;

    Ok(try_join_all(init.into_iter().map(async |r| {
        Ok::<(), Error>({
            let mut tx = conn.begin().await?;
            let got = ReceiptParams::new()
                .r_id(r.id)
                .item_qty(100)
                .update(&mut *tx)
                .await?;
            tx.commit().await?;

            assert_ne!(r, got, "Test if row was updated");
            assert_ne!(
                r.updated_at, got.updated_at,
                "Test if updated_at field was updated"
            );
        })
    }))
    .await?
    .into_iter()
    .collect())
}

#[sqlx::test]
async fn test_rec_param_errors(conn: SqlitePool) -> Result<()> {
    let bad_params = [
        (ReceiptParams::new(), RequestType::Delete, "delete", "id"),
        (ReceiptParams::new(), RequestType::Post, "post", "item id"),
        (
            ReceiptParams::new().item_id(0),
            RequestType::Post,
            "post",
            "item qty",
        ),
        (
            ReceiptParams::new().r_id(0),
            RequestType::Update,
            "update",
            "item id and item qty",
        ),
        (
            ReceiptParams::new().item_id(0),
            RequestType::Get,
            "get",
            "id",
        ),
    ];

    let err_msg = "Test failed: expected error.";

    Ok(try_join_all(bad_params.into_iter().map(
        async |(param, req, req_t, want)| {
            Ok::<(), Error>({
                let mut tx = conn.begin().await?;
                let got = match req {
                    RequestType::Delete => match param.delete(&mut *tx).await {
                        Ok(_) => panic!("{err_msg}"),
                        Err(e) => e.to_string(),
                    },
                    RequestType::Post => match param.post(&mut *tx).await {
                        Ok(_) => panic!("{err_msg}"),
                        Err(e) => e.to_string(),
                    },
                    RequestType::Update => match param.update(&mut *tx).await {
                        Ok(_) => panic!("{err_msg}"),
                        Err(e) => e.to_string(),
                    },
                    RequestType::Get => match param.get(&mut *tx).await {
                        Ok(_) => panic!("{err_msg}"),
                        Err(e) => e.to_string(),
                    },
                    _ => panic!("Test has unhandled req type."),
                };

                assert_eq!(
                    RequestError::missing_param(want).to_string(),
                    got,
                    "Tests if req type {req_t} returned expected error."
                )
            })
        },
    ))
    .await?
    .into_iter()
    .collect())
}
