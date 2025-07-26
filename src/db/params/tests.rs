use super::*;

#[sqlx::test]
async fn test_item_param_errors(conn: SqlitePool) {
    let params = ItemParams::builder()
        .with_item_id(ParamOption::new().map_value(1).to_owned())
        .build();

    if let Err(e) = params.update(&conn).await {
        assert_eq!(
            RequestError::missing_param(
                RequestType::Update,
                "item",
                "item name, item price"
            )
            .to_string(),
            e.to_string(),
            "Test malformed update params."
        )
    }

    if let Err(e) = params.post(&conn).await {
        assert_eq!(
            RequestError::missing_param(RequestType::Post, "item", "item name")
                .to_string(),
            e.to_string(),
            "Test malformed post param"
        )
    }
}

#[sqlx::test]
async fn test_user_param_errors(conn: SqlitePool) {
    let no_id = UserParams::default();
    let no_name = UserParams::builder()
        .with_user_id(ParamOption::new().map_value(0).clone())
        .build();

    if let Err(e) = no_id.delete(&conn).await {
        assert_eq!(
            RequestError::missing_param(RequestType::Delete, "user", "id")
                .to_string(),
            e.to_string(),
            "Test if expected error matches."
        );
    }

    if let Err(e) = no_name.update(&conn).await {
        assert_eq!(
            RequestError::missing_param(RequestType::Update, "user", "name")
                .to_string(),
            e.to_string(),
            "Test if expected error matches."
        )
    }
}

#[sqlx::test]
async fn test_joined_errors(conn: SqlitePool) -> Result<()> {
    let test_cases = [
        (
            JoinedReceiptParams::default(),
            RequestType::Get,
            RequestError::missing_param(
                RequestType::Get,
                "joined receipt",
                "receipt id",
            ),
        ),
        (
            JoinedReceiptParams::builder()
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
            JoinedReceiptParams::default(),
            RequestType::Delete,
            RequestError::missing_param(
                RequestType::Delete,
                "joined receipt",
                "receipt id",
            ),
        ),
        (
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(0).to_owned())
                .build(),
            RequestType::Delete,
            RequestError::not_found(0, "receipts"),
        ),
        (
            JoinedReceiptParams::default(),
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
            RequestError::not_found(0, "receipts"),
        ),
    ];

    for (param, req_type, want) in test_cases {
        let got = match req_type {
            RequestType::Get => param.get(&conn).await.err(),
            RequestType::GetAll => param.get_all(&conn).await.err(),
            RequestType::Post => param.post(&conn).await.err(),
            RequestType::Delete => param.delete(&conn).await.err(),
            RequestType::Update => param.update(&conn).await.err(),
            _ => panic!("Unhandled req type: {req_type}"),
        };

        let message = format!("Test: {req_type} suceeded unexpectedly.");
        let got = got.ok_or(Error::msg(message))?;

        assert_eq!(
            want.to_string(),
            got.to_string(),
            "Test invalid {req_type:?} param returns correct error."
        )
    }
    Ok(())
}
