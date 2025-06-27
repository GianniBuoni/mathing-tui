use super::*;

// TODO
// Add ordering by to the params GET query

#[sqlx::test]
async fn test_join_post(conn: SqlitePool) -> Result<()> {
    init_join_rows(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_join_get_all(conn: SqlitePool) -> Result<()> {
    let want = init_join_rows(&conn).await?;
    let got = JoinedReceiptParams::builder()
        .with_offset(0)
        .build()
        .get_all(&conn)
        .await?;

    assert_eq!(want, got, "Test if get_all matches expected");

    Ok(())
}

#[sqlx::test]
async fn test_join_get(conn: SqlitePool) -> Result<()> {
    let want = init_join_rows(&conn).await?;
    let got = JoinedReceiptParams::builder()
        .with_r_id(
            ParamOption::new()
                .map_value(want.get(0).unwrap().receipt_id)
                .clone(),
        )
        .build()
        .get(&conn)
        .await?;

    assert_eq!(*want.get(0).unwrap(), got, "Test getting joined row by id");

    Ok(())
}

#[sqlx::test]
async fn test_join_delete(conn: SqlitePool) -> Result<()> {
    let initial = init_join_rows(&conn).await?;

    JoinedReceiptParams::builder()
        .with_r_id(
            ParamOption::new()
                .map_value(initial.get(0).unwrap().receipt_id)
                .clone(),
        )
        .build()
        .delete(&conn)
        .await?;

    let got = JoinedReceiptParams::builder()
        .with_offset(0)
        .build()
        .get_all(&conn)
        .await?;
    assert_ne!(initial.len(), got.len(), "Test if entries were deleted.");

    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<()> {
    init_join_rows(&conn).await?;
    // delete Jon
    UserParams::builder()
        .with_user_id(ParamOption::new().map_value(3).clone())
        .build()
        .delete(&conn)
        .await?;

    // PB Pretzel should be deleted
    // Chips and Dip should not be deleted since Noodle is still
    // attached to the receipt
    let got = JoinedReceiptParams::builder()
        .with_offset(0)
        .build()
        .get_all(&conn)
        .await?;
    assert_eq!(2, got.len(), "Test delete cascade for joined rows.");

    Ok(())
}

#[sqlx::test]
async fn test_joined_update(conn: SqlitePool) -> Result<()> {
    let init = init_join_rows(&conn).await?;

    let params = [
        (
            JoinedReceiptParams::builder()
                .with_r_id(
                    ParamOption::new()
                        .map_value(init.get(0).unwrap().receipt_id)
                        .clone(),
                )
                .with_item_qty(ParamOption::new().map_value(1).clone())
                .build(),
            "ID 1, changed qty to 1",
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(
                    ParamOption::new()
                        .map_value(init.get(1).unwrap().receipt_id)
                        .clone(),
                )
                .with_item_id(
                    ParamOption::new()
                        .map_value(init.get(0).unwrap().item_id)
                        .clone(),
                )
                .build(),
            "ID 2, to a differnt item.",
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(
                    ParamOption::new()
                        .map_value(init.get(2).unwrap().receipt_id)
                        .clone(),
                )
                .with_user(3)
                .build(),
            "ID 3, Remove user Noodle from receipt.",
        ),
    ];

    let mut got = vec![];
    for (param, desc) in params.iter() {
        let new = param.update(&conn).await.map_err(|e| {
            let message = format!("{desc} {e}");
            Error::msg(message)
        })?;
        got.push((new, desc));
    }

    got.into_iter()
        .zip(init)
        .for_each(|((new, desc), old)| assert_ne!(old, new, "{desc}"));

    Ok(())
}

#[sqlx::test]
async fn test_joined_reset(conn: SqlitePool) -> Result<()> {
    init_join_rows(&conn).await?;
    let rows = JoinedReceiptParams::builder().build().reset(&conn).await?;
    assert_eq!(3, rows, "Test if expected amount of rows were affected.");

    let got = JoinedReceiptParams::builder()
        .with_offset(0)
        .build()
        .get_all(&conn)
        .await?;
    assert_eq!(0, got.len(), "Test if reset deleted all receipt records.");
    Ok(())
}

#[sqlx::test]
async fn test_joined_errors(conn: SqlitePool) -> Result<()> {
    let test_cases = [
        (
            JoinedReceiptParams::builder()
                .with_item_id(ParamOption::new().map_value(0).clone())
                .with_user(0)
                .build(),
            RequestType::Get,
            RequestError::missing_param(
                RequestType::Get,
                "joined receipt",
                "receipt id",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::GetAll,
            RequestError::missing_param(
                RequestType::GetAll,
                "joined receipt",
                "offset",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .with_item_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Post,
            RequestError::missing_param(
                RequestType::Post,
                "joined receipt",
                "item qty",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .with_item_qty(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Post,
            RequestError::missing_param(
                RequestType::Post,
                "joined receipt",
                "item id",
            ),
        ),
        (
            JoinedReceiptParams::builder().build(),
            RequestType::Delete,
            RequestError::missing_param(
                RequestType::Delete,
                "joined receipt",
                "receipt id",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Delete,
            RequestError::not_found(0, "receipts"),
        ),
        (
            JoinedReceiptParams::builder().build(),
            RequestType::Update,
            RequestError::missing_param(
                RequestType::Update,
                "joined receipt",
                "receipt id",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Update,
            RequestError::missing_param(
                RequestType::Update,
                "joined receipt",
                "item id, item qty, or users",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).clone())
                .with_user(0)
                .build(),
            RequestType::Update,
            RequestError::not_found(0, "receipts_users"),
        ),
    ];

    for (param, req_type, want) in test_cases {
        let got = match req_type {
            RequestType::Get => {
                param.get(&conn).await.err().unwrap().to_string()
            }
            RequestType::GetAll => {
                param.get_all(&conn).await.err().unwrap().to_string()
            }
            RequestType::Post => {
                param.post(&conn).await.err().unwrap().to_string()
            }
            RequestType::Delete => {
                param.delete(&conn).await.err().unwrap().to_string()
            }
            RequestType::Update => {
                param.update(&conn).await.err().unwrap().to_string()
            }
            _ => panic!("unhandled error, check test's req type"),
        };

        assert_eq!(
            want.to_string(),
            got,
            "Test invalid {req_type:?} param returns correct error."
        )
    }

    Ok(())
}
