use super::*;
use crate::prelude::{Request, RequestError};

async fn init_test(conn: &SqlitePool) -> Result<()> {
    let users = try_join_all(TEST_USERS.into_iter().map(async |name| {
        anyhow::Ok::<StoreUser>({
            UserParams::new().user_name(name).post(&conn).await?
        })
    }))
    .await?
    .into_iter()
    .collect::<Vec<StoreUser>>();

    let receipts =
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, qty)| {
            anyhow::Ok::<StoreReceipt>({
                let item = ItemParams::new()
                    .item_name(name)
                    .item_price(price)
                    .post(&conn)
                    .await?;

                let mut tx = conn.begin().await?;
                let receipts = ReceiptParams::new()
                    .item_id(item.id)
                    .item_qty(qty)
                    .post(&mut *tx)
                    .await?;
                tx.commit().await?;

                receipts
            })
        }))
        .await?
        .into_iter()
        .collect::<Vec<StoreReceipt>>();

    let err = "Wrong id/user match; check how you add users into db.";
    let noodle = if users.get(1).unwrap().name == "Noodle" {
        2 as i64
    } else {
        return Err(Error::msg(err));
    };
    let jon = if users.get(2).unwrap().name == "Jon" {
        3 as i64
    } else {
        return Err(Error::msg(err));
    };

    try_join_all(receipts.into_iter().map(async |r| {
        Ok::<(), Error>({
            let assignments = match r.id {
                1 => vec![jon],
                2 => vec![noodle],
                3 => vec![noodle, jon],
                _ => vec![],
            };

            try_join_all(assignments.into_iter().map(async |u_id| {
                Ok::<(), Error>({
                    let mut tx = conn.begin().await?;
                    ReceiptsUsersParams::new()
                        .u_id(u_id)
                        .r_id(r.id)
                        .post(&mut *tx)
                        .await?;
                    tx.commit().await?;
                })
            }))
            .await?;
        })
    }))
    .await?;

    Ok(())
}

#[sqlx::test]
async fn test_add_receipts_users(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_get_receipts_users(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;

    let got = ReceiptsUsersParams::new()
        .r_id(3)
        .get(&mut *conn.acquire().await?)
        .await?;

    assert_eq!(
        2,
        got.len(),
        "Test if getting receipts_users returnt righ amount of rows."
    );

    Ok(())
}

#[sqlx::test]
async fn test_del_receipts_users(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;

    let (test, want) = (ReceiptsUsersParams::new().r_id(1).u_id(3), 1 as u64);

    let mut tx = conn.begin().await?;
    let got_1 = test.delete(&mut *tx).await?;
    assert_eq!(want, got_1, "Test delete receipts_users params.");

    Ok(())
}

#[sqlx::test]
async fn test_del_cascade(conn: SqlitePool) -> Result<()> {
    init_test(&conn).await?;

    let mut tx = conn.begin().await?;
    ReceiptParams::new().r_id(3).delete(&mut *tx).await?;
    tx.commit().await?;

    //TODO get accurate amount of rows deleted when a receipt is deleted;

    match ReceiptsUsersParams::new()
        .r_id(3)
        .get(&mut *conn.acquire().await?)
        .await
    {
        Ok(e) => {
            panic!("Test expeceted an error, delete cascade failed. Got: {e:?}")
        }
        Err(e) => assert_eq!(
            RequestError::not_found(3, "receipts_users").to_string(),
            e.to_string()
        ),
    }
    Ok(())
}
