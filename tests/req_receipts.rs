use common::prelude::*;

mod common;

#[sqlx::test]
fn test_handle_receipt_req(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let payloads = [
        (
            RequestType::Post,
            JoinedReceiptParams::builder()
                .with_item_id(ParamOption::new().map_value(1).to_owned())
                .with_item_qty(ParamOption::new().map_value(1).to_owned())
                .with_user(1)
                .build(),
        ),
        (
            RequestType::Get,
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(2).to_owned())
                .build(),
        ),
        (
            RequestType::Update,
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(3).to_owned())
                .with_item_qty(ParamOption::new().map_value(2).to_owned())
                .with_user(1)
                .build(),
        ),
    ];
    // (item_id, item_qty, user_count)
    let want = [(1, 1, 1), (2, 1, 1), (3, 2, 1)];

    try_join_all({
        payloads.into_iter().map(async |(req_type, payload)| {
            let req = DbRequest::new()
                .with_req_type(req_type)
                .with_payload(DbPayload::ReceiptParams(payload));
            let DbPayload::Receipt(res) =
                handle_requests(req, &conn).await.payload
            else {
                let msg = format!("{req_type} should output Receipt payload.");
                return Err(Error::msg(msg));
            };
            Aok::<StoreJoinRow>(res)
        })
    })
    .await?
    .into_iter()
    .zip(want)
    .for_each(|(got, want)| {
        assert_eq!(want.0, got.item_id, "Test req outputs expected item id.");
        assert_eq!(want.1, got.item_qty, "Test req outputs expected item qty.");
        assert_eq!(
            want.2,
            got.users.len(),
            "Test req outputs expected user lenght."
        );
    });

    Ok(())
}

#[sqlx::test]
async fn test_delete_receipt_req(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let req = DbRequest::new()
        .with_req_type(RequestType::Delete)
        .with_payload(DbPayload::ReceiptParams(
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(3).to_owned())
                .build(),
        ));

    let DbPayload::AffectedRows(_, res) =
        handle_requests(req, &conn).await.payload
    else {
        let msg = "Delete request should output AffectedRow payload.";
        return Err(Error::msg(msg));
    };

    assert_eq!(1, res, "Test delete req output correct payload.");
    Ok(())
}
