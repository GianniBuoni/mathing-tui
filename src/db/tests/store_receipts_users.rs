use super::*;

use store_receits_users_init::init_test;

#[sqlx::test]
async fn test_add_receipts_users(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    assert!(init_test(&conn).await.is_ok(), "Test if as");
    Ok(())
}

#[sqlx::test]
async fn test_get_reciepts_users_raw(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let raw_rows = get_store_joined_raw(&conn, 0).await?;

    assert_eq!(raw_rows.len(), 3);

    let want = expected_sql_rows();
    want.into_iter().zip(raw_rows).for_each(|(want, got)| {
        assert_eq!(want, got);
    });

    Ok(())
}

#[sqlx::test]
async fn test_get_receipts_joined(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let users = get_store_users(&conn).await?;

    let want = expected_joined_rows(&users);
    let got = get_store_joined_rows(&conn, 0).await?;

    want.into_iter().zip(got).for_each(|(want, got)| {
        assert_eq!(want, got);
    });

    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let users = get_store_users(&conn).await?;
    let want = &expected_joined_rows(&users)[0];

    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        TEST_ITEMS.len(),
        "Test initial returned length is expected."
    );

    //delete user Noodle
    delete_store_user(&conn, 2).await?;

    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        2,
        "Deleteing a user should've affected returned rows"
    );

    // delete item Chips and Dip
    delete_store_item(&conn, 3).await?;

    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        1,
        "Deleting an item should have affected returned rows"
    );
    assert_eq!(*want, rows[0]);

    Ok(())
}

#[sqlx::test]
async fn test_reset_cascades(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(rows.len(), TEST_ITEMS.len(), "Confirm test cases exist.");

    delete_store_receipts(&conn).await?;
    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        0,
        "Reset receipts table should have reset join table as well."
    );

    Ok(())
}

#[sqlx::test]
async fn test_offset(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let rows = get_store_joined_rows(&conn, 1).await?;
    assert_eq!(rows.len(), 2, "Test if offset by 1 affects returned rows.");

    Ok(())
}

#[sqlx::test]
async fn test_delete_receipts_users(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    // remove Noodle from third test case
    delete_store_receipts_users(&conn, 3, 2).await?;

    let rows = get_store_joined_rows(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        TEST_ITEMS.len(),
        "Returned rows shouldn't be affected by removing one user from receipt"
    );

    let jon = get_store_user_single(&conn, 3).await?;
    let want = StoreJoinRow {
        receipt_id: 3,
        item_id: 3,
        users: vec![jon],
        item_name: "Chips and Dip".into(),
        item_qty: 3,
        item_price: 5.55,
        user_count: 1,
    };

    assert_eq!(want, rows[2]);

    Ok(())
}

#[sqlx::test]
async fn test_get_totals(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let want = expected_totals();
    let mut got = StoreTotal::default();

    get_store_joined_rows(&conn, 0)
        .await?
        .into_iter()
        .zip(intermediate_totals())
        .for_each(|(row, want)| {
            assert_eq!(want, row.calc());
            got.add(row.calc());
        });

    assert_eq!(want, got.0, "Test if all the math is right ✨");

    Ok(())
}
