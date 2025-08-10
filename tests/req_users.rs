use common::prelude::*;

mod common;

#[sqlx::test]
async fn test_handle_user_req(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let payloads = [
        (
            RequestType::Post,
            UserParams::builder()
                .with_user_name(ParamOption::new().map_value("Jon").to_owned())
                .build(),
        ),
        (
            RequestType::Get,
            UserParams::builder()
                .with_user_id(ParamOption::new().map_value(1).to_owned())
                .build(),
        ),
        (
            RequestType::Update,
            UserParams::builder()
                .with_user_id(ParamOption::new().map_value(2).to_owned())
                .with_user_name(
                    ParamOption::new().map_value("new name").to_owned(),
                )
                .build(),
        ),
    ];
    let want = ["Jon", "Thing", "new name"];

    try_join_all(payloads.into_iter().map(async |(req_type, payload)| {
        let req = DbRequest::new()
            .with_req_type(req_type)
            .with_payload(DbPayload::UserParams(payload));
        let DbPayload::User(res) = handle_requests(req, &conn).await.payload
        else {
            let msg = format!("{req_type} should output a StoreUser.");
            return Err(Error::msg(msg));
        };
        Aok::<StoreUser>(res)
    }))
    .await?
    .into_iter()
    .zip(want)
    .for_each(|(got, want)| {
        assert_eq!(want, got.name, "Test if output name matches.");
    });

    Ok(())
}

#[sqlx::test]
async fn test_handle_delete_user(conn: SqlitePool) -> Result<()> {
    try_init_test_db(&conn).await?;

    let req = DbRequest::new()
        .with_req_type(RequestType::Delete)
        .with_payload(DbPayload::UserParams(
            UserParams::builder()
                .with_user_id(ParamOption::new().map_value(1).to_owned())
                .build(),
        ));

    let DbPayload::AffectedRows(_, res) =
        handle_requests(req, &conn).await.payload
    else {
        let msg = "Delete request should output AffectedRows payload.";
        return Err(Error::msg(msg));
    };

    assert_eq!(1, res, "Test if Delete req outputs correct payload.");

    Ok(())
}
