use super::*;

// TODO
// Add ordering by to the params GET query

async fn join_init_test(conn: &SqlitePool) -> Result<Vec<StoreJoinRow>> {
    let users = init_users(conn).await?;
    let items = intit_items(conn).await?;
    let mut res = vec![];

    for (index, r) in items.into_iter().enumerate() {
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        let mut param = JoinedReceiptParams::builder();
        param
            .item_id(ParamOption::new().map_value(r.0.id).clone())
            .item_qty(ParamOption::new().map_value(r.1).clone())
            .offset(0);

        match index {
            0 => {
                param.add_user(users.get(2).unwrap().id);
            }
            1 => {
                param.add_user(users.get(1).unwrap().id);
            }
            2 => {
                param
                    .add_user(users.get(1).unwrap().id)
                    .add_user(users.get(2).unwrap().id);
            }
            _ => {}
        }
        res.push(param.build().post(conn).await?);
    }

    Ok(res)
}

#[sqlx::test]
async fn test_join_post(conn: SqlitePool) -> Result<()> {
    join_init_test(&conn).await?;
    Ok(())
}

#[sqlx::test]
async fn test_join_get_all(conn: SqlitePool) -> Result<()> {
    let want = join_init_test(&conn).await?;
    let got = JoinedReceiptParams::builder()
        .offset(0)
        .build()
        .get_all(&conn)
        .await?;

    assert_eq!(want, got, "Test if get_all matches expected");

    Ok(())
}

#[sqlx::test]
async fn test_join_get(conn: SqlitePool) -> Result<()> {
    let want = join_init_test(&conn).await?;
    let got = JoinedReceiptParams::builder()
        .r_id(
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
    let initial = join_init_test(&conn).await?;

    JoinedReceiptParams::builder()
        .r_id(
            ParamOption::new()
                .map_value(initial.get(0).unwrap().receipt_id)
                .clone(),
        )
        .build()
        .delete(&conn)
        .await?;

    let got = JoinedReceiptParams::builder()
        .offset(0)
        .build()
        .get_all(&conn)
        .await?;
    assert_ne!(initial.len(), got.len(), "Test if entries were deleted.");

    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<()> {
    join_init_test(&conn).await?;
    UserParams::builder()
        .user_id(ParamOption::new().map_value(3).clone())
        .build()
        .delete(&conn)
        .await?;

    let got = JoinedReceiptParams::builder()
        .offset(0)
        .build()
        .get_all(&conn)
        .await?;
    assert_eq!(1, got.len(), "Test delete cascade for joined rows.");

    Ok(())
}

#[sqlx::test]
async fn test_joined_update(conn: SqlitePool) -> Result<()> {
    let init = join_init_test(&conn).await?;

    let params = [
        (
            JoinedReceiptParams::builder()
                .r_id(
                    ParamOption::new()
                        .map_value(init.get(0).unwrap().receipt_id)
                        .clone(),
                )
                .item_qty(ParamOption::new().map_value(1).clone())
                .build(),
            "ID 1, changed qty to 1",
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(
                    ParamOption::new()
                        .map_value(init.get(1).unwrap().receipt_id)
                        .clone(),
                )
                .item_id(
                    ParamOption::new()
                        .map_value(init.get(0).unwrap().item_id)
                        .clone(),
                )
                .build(),
            "ID 2, to a differnt item.",
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(
                    ParamOption::new()
                        .map_value(init.get(2).unwrap().receipt_id)
                        .clone(),
                )
                .add_user(3)
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
    join_init_test(&conn).await?;
    let rows = JoinedReceiptParams::builder().build().reset(&conn).await?;
    assert_eq!(3, rows, "Test if expected amount of rows were affected.");

    let got = JoinedReceiptParams::builder()
        .offset(0)
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
                .item_id(ParamOption::new().map_value(0).clone())
                .add_user(0)
                .build(),
            RequestType::Get,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::GetAll,
            RequestError::missing_param("offset"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .item_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Post,
            RequestError::missing_param("item qty"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .item_qty(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Post,
            RequestError::missing_param("item id"),
        ),
        (
            JoinedReceiptParams::builder().build(),
            RequestType::Delete,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Delete,
            RequestError::not_found(0, "receipts"),
        ),
        (
            JoinedReceiptParams::builder().build(),
            RequestType::Update,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .build(),
            RequestType::Update,
            RequestError::missing_param("item id, item qty, or users"),
        ),
        (
            JoinedReceiptParams::builder()
                .r_id(ParamOption::new().map_value(0).clone())
                .add_user(0)
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

#[sqlx::test]
async fn test_get_totals(conn: SqlitePool) -> Result<()> {
    join_init_test(&conn).await?;
    let want = expected_totals();
    let mut got = StoreTotal::default();

    JoinedReceiptParams::builder()
        .offset(0)
        .build()
        .get_all(&conn)
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
