use core::panic;

use ratatui::text::ToText;

use super::*;

// TODO: make these tests more deterministic.
// 2. Add ordering by to the query

async fn join_init_test(conn: &SqlitePool) -> Result<Vec<StoreJoinRow>> {
    let users = init_users(conn).await?;
    let items = intit_items(conn).await?;
    let mut res = vec![];

    for (index, r) in items.into_iter().enumerate() {
        sleep_until(Instant::now() + Duration::from_secs(1)).await;
        let mut param = JoinedReceiptParams::new()
            .item_id(r.0.id)
            .item_qty(r.1)
            .offset(0);

        match index {
            0 => {
                param = param.add_user(users.get(2).unwrap().id);
            }
            1 => {
                param = param.add_user(users.get(1).unwrap().id);
            }
            2 => {
                param = param
                    .add_user(users.get(1).unwrap().id)
                    .add_user(users.get(2).unwrap().id);
            }
            _ => {}
        }
        res.push(param.post(conn).await?);
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
    let got = JoinedReceiptParams::new().offset(0).get_all(&conn).await?;

    assert_eq!(want, got, "Test if get_all matches expected");

    Ok(())
}

#[sqlx::test]
async fn test_join_get(conn: SqlitePool) -> Result<()> {
    let want = join_init_test(&conn).await?;
    let got = JoinedReceiptParams::new()
        .r_id(want.get(0).unwrap().receipt_id)
        .get(&conn)
        .await?;

    assert_eq!(*want.get(0).unwrap(), got, "Test getting joined row by id");

    Ok(())
}

#[sqlx::test]
async fn test_join_delete(conn: SqlitePool) -> Result<()> {
    let initial = join_init_test(&conn).await?;

    JoinedReceiptParams::new()
        .r_id(initial.get(0).unwrap().receipt_id)
        .delete(&conn)
        .await?;

    let got = JoinedReceiptParams::new().offset(0).get_all(&conn).await?;
    assert_ne!(initial.len(), got.len(), "Test if entries were deleted.");

    Ok(())
}

#[sqlx::test]
async fn test_delete_cascade(conn: SqlitePool) -> Result<()> {
    join_init_test(&conn).await?;
    UserParams::new().user_id(3).delete(&conn).await?;

    let got = JoinedReceiptParams::new().offset(0).get_all(&conn).await?;
    assert_eq!(1, got.len(), "Test delete cascade for joined rows.");

    Ok(())
}

#[sqlx::test]
async fn test_joined_update(conn: SqlitePool) -> Result<()> {
    let init = join_init_test(&conn).await?;

    let params = [
        (
            JoinedReceiptParams::new()
                .r_id(init.get(0).unwrap().receipt_id)
                .item_qty(1),
            "ID 1, changed qty to 1",
        ),
        (
            JoinedReceiptParams::new()
                .r_id(init.get(1).unwrap().receipt_id)
                .item_id(init.get(0).unwrap().item_id),
            "ID 2, to a differnt item.",
        ),
        (
            JoinedReceiptParams::new()
                .r_id(init.get(2).unwrap().receipt_id)
                .add_user(3),
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
async fn test_joined_errors(conn: SqlitePool) -> Result<()> {
    let test_cases = [
        (
            JoinedReceiptParams::new().item_id(0).add_user(0),
            RequestType::Get,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::new().r_id(0),
            RequestType::GetAll,
            RequestError::missing_param("offset"),
        ),
        (
            JoinedReceiptParams::new().r_id(0).item_id(0),
            RequestType::Post,
            RequestError::missing_param("item qty"),
        ),
        (
            JoinedReceiptParams::new().r_id(0).item_qty(0),
            RequestType::Post,
            RequestError::missing_param("item id"),
        ),
        (
            JoinedReceiptParams::new(),
            RequestType::Delete,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::new().r_id(0),
            RequestType::Delete,
            RequestError::not_found(0, "receipts"),
        ),
        (
            JoinedReceiptParams::new(),
            RequestType::Update,
            RequestError::missing_param("receipt id"),
        ),
        (
            JoinedReceiptParams::new().r_id(0),
            RequestType::Update,
            RequestError::missing_param("item id, item qty, or users"),
        ),
        (
            JoinedReceiptParams::new().r_id(0).add_user(0),
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
