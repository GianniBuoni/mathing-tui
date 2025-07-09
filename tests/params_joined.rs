use futures::future::try_join_all;

use common::*;

mod common;

#[sqlx::test]
async fn test_join_post(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    try_join_all({
        MOCK_RECEIPTS.into_iter().zip(MOCK_USERS).map(
            async |((r_id, item_id, item_qty), (u_id, _))| {
                JoinedReceiptParams::builder()
                    .with_r_id(ParamOption::new().map_value(r_id).to_owned())
                    .with_item_id(
                        ParamOption::new().map_value(item_id).to_owned(),
                    )
                    .with_item_qty(
                        ParamOption::new().map_value(item_qty).to_owned(),
                    )
                    .with_user(u_id)
                    .build()
                    .post(&conn)
                    .await
            },
        )
    })
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_join_get(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let (r_id, item_id, item_qty) = MOCK_RECEIPTS
        .get(0)
        .ok_or(Error::msg("Could not unwrap \"want\" variable."))?;

    let got = JoinedReceiptParams::builder()
        .with_r_id(ParamOption::new().map_value(1).clone())
        .build()
        .get(&conn)
        .await?;

    assert_eq!(*r_id, got.receipt_id, "Test retreieved ids match.");
    assert_eq!(*item_id, got.item_id, "Test retrieved item ids match.");
    assert_eq!(
        *item_qty, got.item_qty,
        "Test if retrieved item_qty's match."
    );
    Ok(())
}

#[sqlx::test]
async fn test_join_get_all(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let want = MOCK_RECEIPTS;

    JoinedReceiptParams::default()
        .with_offset(0)
        .get_all(&conn)
        .await?
        .into_iter()
        .zip(want)
        .for_each(|(got, (r_id, item_id, item_qty))| {
            assert_eq!(r_id, got.receipt_id, "Test retreieved ids match.");
            assert_eq!(item_id, got.item_id, "Test retrieved item ids match.");
            assert_eq!(
                item_qty, got.item_qty,
                "Test if retrieved item_qty's match."
            );
        });

    Ok(())
}

#[sqlx::test]
async fn test_join_delete(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let want = MOCK_RECEIPTS.len();

    JoinedReceiptParams::builder()
        .with_r_id(ParamOption::new().map_value(1).clone())
        .build()
        .delete(&conn)
        .await?;

    let got = JoinedReceiptParams::default()
        .with_offset(0)
        .get_all(&conn)
        .await?
        .len();

    assert_ne!(want, got, "Test if entries were deleted.");
    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    // delete Jon
    UserParams::builder()
        .with_user_id(ParamOption::new().map_value(3).clone())
        .build()
        .delete(&conn)
        .await?;

    // PB Pretzel should be deleted
    // Chips and Dip should not be deleted since Noodle is still
    // attached to the receipt
    let got = JoinedReceiptParams::default().get_all(&conn).await?.len();
    assert_eq!(2, got, "Test delete cascade for joined rows.");

    Ok(())
}

#[sqlx::test]
async fn test_joined_update(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let init = JoinedReceiptParams::default().get_all(&conn).await?;

    let params = [
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(1).to_owned())
                .with_item_qty(ParamOption::new().map_value(1).to_owned())
                .build(),
            "ID 1, changed qty to 1",
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(2).to_owned())
                .with_item_id(ParamOption::new().map_value(1).to_owned())
                .build(),
            "ID 2, to a differnt item.",
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(3).to_owned())
                .with_user(3)
                .build(),
            "ID 3, Remove user Noodle from receipt.",
        ),
    ];

    try_join_all({
        params
            .into_iter()
            .zip(init)
            .map(async |((param, desc), old)| {
                Aok((param.update(&conn).await?, old, desc))
            })
    })
    .await?
    .into_iter()
    .for_each(|(new, old, desc)| assert_ne!(old, new, "{desc}"));

    Ok(())
}

#[sqlx::test]
async fn test_joined_reset(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let rows = JoinedReceiptParams::default().reset(&conn).await?;
    assert_eq!(3, rows, "Test if expected amount of rows were affected.");

    let got = JoinedReceiptParams::default().get_all(&conn).await?;
    assert_eq!(0, got.len(), "Test if reset deleted all receipt records.");

    Ok(())
}

#[sqlx::test]
async fn test_join_count(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let got = JoinedReceiptParams::default().count(&conn).await?;
    assert_eq!(3, got, "Test if row count matches expected value.");

    Ok(())
}
