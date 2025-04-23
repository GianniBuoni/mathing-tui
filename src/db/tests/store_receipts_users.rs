use super::*;

use store_receits_users_init::{TEST_ITEMS, init_test, want};

#[sqlx::test]
async fn test_add_receipts_users(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_get_receipts_joined(
    conn: SqlitePool,
) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let want = want();
    let got = get_store_receipts_joined(&conn, 0).await?;

    for (want, got) in want.iter().zip(got) {
        assert_eq!(*want, got);
    }

    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;

    let rows = get_store_receipts_joined(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        TEST_ITEMS.len(),
        "Test initial returnd length is expected."
    );

    //delete user Noodle
    delete_store_user(&conn, 2).await?;

    let rows = get_store_receipts_joined(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        2,
        "Deleted user should have affected returned rows."
    );

    // delete item Chips and Dip
    delete_store_item(&conn, 3).await?;

    let want = &want()[0];

    let rows = get_store_receipts_joined(&conn, 0).await?;
    assert_eq!(
        rows.len(),
        1,
        "Deleted item should have affected returned rows"
    );
    assert_eq!(*want, rows[0]);

    Ok(())
}

#[sqlx::test]
async fn test_offset(conn: SqlitePool) -> Result<(), Box<dyn Error>> {
    init_test(&conn).await?;
    let rows = get_store_receipts_joined(&conn, 1).await?;
    assert_eq!(rows.len(), 2, "Test if offset by 1 affects returned rows.");

    Ok(())
}
