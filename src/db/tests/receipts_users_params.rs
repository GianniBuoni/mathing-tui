use super::*;
use crate::prelude::{Request, RequestError};

async fn init_test(conn: &SqlitePool) -> Result<()> {
    let users = try_join_all(TEST_USERS.into_iter().map(async |name| {
        Ok::<StoreUser, Error>({
            let mut tx = conn.begin().await?;
            let users =
                UserParams::new().user_name(name).post(&mut *tx).await?;
            tx.commit().await?;
            users
        })
    }))
    .await?
    .into_iter()
    .collect::<Vec<StoreUser>>();

    let receipts =
        try_join_all(TEST_ITEMS.into_iter().map(async |(name, price, qty)| {
            Ok::<StoreReceipt, Error>({
                let mut tx = conn.begin().await?;
                let item = ItemParams::new()
                    .item_name(name)
                    .item_price(price)
                    .post(&mut *tx)
                    .await?;
                tx.commit().await?;

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

    let params = [
        ReceiptsUsersParams::new().r_id(3).u_id(2),
        ReceiptsUsersParams::new().r_id(3).u_id(3),
    ];

    try_join_all(params.into_iter().map(async |param| {
        Ok::<(), Error>({
            let mut tx = conn.begin().await?;
            param.get(&mut *tx).await?;
        })
    }))
    .await?;

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

    let params = [
        ReceiptsUsersParams::new().r_id(3).u_id(2),
        ReceiptsUsersParams::new().r_id(3).u_id(3),
    ];

    for param in params.into_iter() {
        let mut tx = conn.begin().await?;
        let got = param.get(&mut *tx).await;

        let id = format!(
            "receipt_id:{}, user_id:{}",
            param.r_id.unwrap(),
            param.u_id.unwrap()
        );

        match got {
            Ok(_) => panic!("Test expeceted an error, delete cascade failed."),
            Err(e) => assert_eq!(
                RequestError::not_found(id, "receipts_users").to_string(),
                e.to_string()
            ),
        }
    }

    Ok(())
}
