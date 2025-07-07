use std::collections::HashMap;

use common::try_init_test_db;
use mathing_tui::prelude::*;
use rust_decimal::dec;
use sqlx::SqlitePool;

mod common;

fn expected_totals() -> StoreTotal {
    StoreTotal::from(HashMap::from([(3, dec!(18.30)), (2, dec!(17.81))]))
}

#[sqlx::test]
async fn test_totals_adding(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let want = expected_totals();

    let got = JoinedReceiptParams::default()
        .get_all(&conn)
        .await?
        .iter_mut()
        .try_fold(StoreTotal::default(), |mut acc, row| {
            acc.add(row.try_calc()?);
            Aok(acc)
        })?;

    assert_eq!(want, got, "Test if all the math is right ✨");
    Ok(())
}
