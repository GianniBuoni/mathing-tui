use core::panic;

use super::*;

#[sqlx::test]
async fn test_req_errors(conn: SqlitePool) {
    let test_cases = [
        (
            DbPayload::None,
            RequestType::Get,
            RequestError::unhandled("payload", "None"),
            "Test unhandled None payload.",
        ),
        (
            DbPayload::Item(StoreItem::default()),
            RequestType::Post,
            RequestError::unhandled("payload", "Item"),
            "Test unhandled Item payload.",
        ),
        (
            DbPayload::User(StoreUser::default()),
            RequestType::Post,
            RequestError::unhandled("payload", "User"),
            "Test unhandled User payload.",
        ),
        (
            DbPayload::Receipt(StoreJoinRow::default()),
            RequestType::Post,
            RequestError::unhandled("payload", "Receipt"),
            "Test unhandled Receipt payload.",
        ),
        (
            DbPayload::AffectedRows(0),
            RequestType::Post,
            RequestError::unhandled("payload", "AffectedRows"),
            "Test unhandled AffectedRow payload.",
        ),
        (
            DbPayload::ItemParams(ItemParams::new()),
            RequestType::None,
            RequestError::unhandled("request type", "None"),
            "Test unhandled None request type.",
        ),
        (
            DbPayload::ItemParams(ItemParams::new()),
            RequestType::Reset,
            RequestError::unhandled("request type", "Reset"),
            "Test unhandled DeleteAll request type.",
        ),
        (
            DbPayload::ItemParams(ItemParams::new()),
            RequestType::Post,
            RequestError::missing_param("item name"),
            "Test invalid request params.",
        ),
        (
            DbPayload::ItemParams(ItemParams::new().item_id(1)),
            RequestType::Get,
            RequestError::not_found(1, "items"),
            "Test vaild params with DB error.",
        ),
    ];

    for (payload, req_type, want, desc) in test_cases {
        let got = handle_requests(
            DbRequest::new().payload(payload).req_type(req_type),
            &conn,
        )
        .await;

        if got.error.is_none() {
            panic!("Tests expected to return a response with an error.")
        }

        assert_eq!(want.to_string(), got.error.unwrap(), "{desc}");
    }
}
