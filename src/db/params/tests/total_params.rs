use super::*;

#[sqlx::test]
async fn test_get_prices(conn: SqlitePool) -> Result<()> {
    init_join_rows(&conn).await?;
    let want = StoreJoinPrices::mock();
    let got = TotalsParams::get_prices(&conn).await?;

    assert_eq!(
        want, got,
        "Test if get total method returns expected values."
    );

    Ok(())
}

#[sqlx::test]
async fn test_get_total(conn: SqlitePool) -> Result<()> {
    init_join_rows(&conn).await?;
    let want = StoreTotal::mock();
    let got = TotalsParams::get_total(&conn).await?;

    assert_eq!(
        want, got,
        "Test if params methods calculate totals correctly."
    );

    Ok(())
}
