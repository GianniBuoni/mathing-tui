use common::prelude::*;

mod common;

#[sqlx::test]
async fn test_handle_item_requests(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let payloads = [
        (
            RequestType::Post,
            ItemParams::builder()
                .with_item_price(ParamOption::new().map_value(9.49).to_owned())
                .with_item_name(
                    ParamOption::new().map_value("Salmon").to_owned(),
                )
                .build(),
        ),
        (
            RequestType::Get,
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(1).to_owned())
                .build(),
        ),
        (
            RequestType::Update,
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(2).to_owned())
                .with_item_name(
                    ParamOption::new().map_value("new name").to_owned(),
                )
                .with_item_price(ParamOption::new().map_value(0.).to_owned())
                .build(),
        ),
    ];
    let want = [("Salmon", 9.49), ("PB Prezel", 4.99), ("new name", 0.)];

    try_join_all(payloads.into_iter().map(async |(req_type, payload)| {
        let req = DbRequest::new()
            .with_req_type(req_type)
            .with_payload(DbPayload::ItemParams(payload));
        let DbPayload::Item(res) = handle_requests(req, &conn).await.payload
        else {
            let msg = format!("{req_type} should output a StoreItem payload.");
            return Err(Error::msg(msg));
        };
        Aok::<StoreItem>(res)
    }))
    .await?
    .into_iter()
    .zip(want)
    .for_each(|(got, want)| {
        assert_eq!(want.0, got.name, "Test if output name matches expected.");
        assert_eq!(want.1, got.price, "Test if output price matches expected.");
    });

    Ok(())
}

#[sqlx::test]
async fn test_handle_delete_item_req(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;
    let req = DbRequest::new()
        .with_req_type(RequestType::Delete)
        .with_payload(DbPayload::ItemParams(
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(1).to_owned())
                .build(),
        ));

    let DbPayload::AffectedRows(res) =
        handle_requests(req, &conn).await.payload
    else {
        let message = "Delete request should output an AffectedRows payload";
        return Err(Error::msg(message));
    };

    assert_eq!(1, res, "Test handling delete request.");

    Ok(())
}
