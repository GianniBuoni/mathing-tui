use core::panic;

use super::*;

use DbPayload::{
    Item, ItemParams as ReqItem, Receipt, ReceiptParams as ReqReceipt, User,
    UserParams as ReqUser,
};

#[sqlx::test]
async fn test_req_handler_items(conn: SqlitePool) {
    let test_cases = [
        (
            ReqItem(ItemParams::new().item_name("Slamon").item_price(9.49)),
            RequestType::Post,
            Item(StoreItem {
                name: "Slamon".into(),
                price: 9.49,
                ..Default::default()
            }),
        ),
        (
            ReqItem(ItemParams::new().item_id(1)),
            RequestType::Get,
            Item(StoreItem {
                name: "Slamon".into(),
                price: 9.49,
                ..Default::default()
            }),
        ),
        (
            ReqItem(
                ItemParams::new()
                    .item_id(1)
                    .item_name("New name")
                    .item_price(0.),
            ),
            RequestType::Update,
            Item(StoreItem {
                name: "New name".into(),
                price: 0.,
                ..Default::default()
            }),
        ),
        (
            ReqItem(ItemParams::new().item_id(1)),
            RequestType::Delete,
            DbPayload::AffectedRows(1),
        ),
    ];

    // desired payload
    for (payload, req_type, want) in test_cases {
        let got = handle_requests(
            DbRequest::new().req_type(req_type).payload(payload),
            &conn,
        )
        .await
        .payload;

        match got {
            Item(got) => {
                if let Item(want) = want {
                    assert_eq!(
                        want.name, got.name,
                        "Test if item req outputs expected name."
                    );
                    assert_eq!(
                        want.price, got.price,
                        "Test if item req output expected price."
                    )
                } else {
                    panic!("Test arm did't expect StoreItem as the DbPayload.")
                }
            }
            DbPayload::AffectedRows(_) => {
                assert_eq!(want, got, "Test if the req deletes expected rows.")
            }
            _ => panic!(
                "Test expected a StoreItem or AffectedRows as the DbPaylod."
            ),
        };
    }
}

#[sqlx::test]
fn test_req_handler_users(conn: SqlitePool) {
    let test_cases = [
        (
            ReqUser(UserParams::new().user_name("Jon")),
            RequestType::Post,
            User(StoreUser {
                name: "Jon".into(),
                ..Default::default()
            }),
        ),
        (
            ReqUser(UserParams::new().user_id(1)),
            RequestType::Get,
            User(StoreUser {
                name: "Jon".into(),
                ..Default::default()
            }),
        ),
        (
            ReqUser(UserParams::new().user_id(1).user_name("Noodle")),
            RequestType::Update,
            User(StoreUser {
                name: "Noodle".into(),
                ..Default::default()
            }),
        ),
        (
            ReqUser(UserParams::new().user_id(1)),
            RequestType::Delete,
            DbPayload::AffectedRows(1),
        ),
    ];

    for (payload, req_type, want) in test_cases {
        let got = handle_requests(
            DbRequest::new().req_type(req_type).payload(payload),
            &conn,
        )
        .await
        .payload;

        match got {
            User(got) => {
                if let User(want) = want {
                    assert_eq!(
                        want.name, got.name,
                        "Test if req responds w/ expected user."
                    )
                } else {
                    panic!("Test arm didn't expect StoreUser as DbPayload.")
                }
            }
            DbPayload::AffectedRows(_) => {
                assert_eq!(want, got, "Test if req deletes expected rows.")
            }
            _ => panic!(
                "Test expected StoreUser or AffectedRows as the DbPayload."
            ),
        }
    }
}

#[sqlx::test]
async fn test_req_handler_receipts(conn: SqlitePool) -> Result<()> {
    // init test
    ItemParams::new()
        .item_name("Slamon")
        .item_price(9.49)
        .post(&conn)
        .await?;

    UserParams::new().user_name("Jon").post(&conn).await?;

    let test_cases = [
        (
            ReqReceipt(
                JoinedReceiptParams::new()
                    .add_user(1)
                    .item_id(1)
                    .item_qty(1),
            ),
            RequestType::Post,
            Receipt(StoreJoinRow {
                users: vec![StoreUser {
                    name: "Jon".into(),
                    ..Default::default()
                }],
                item_name: "Slamon".into(),
                user_count: 1,
                item_id: 1,
                item_price: 9.49,
                item_qty: 1,
                ..Default::default()
            }),
        ),
        (
            ReqReceipt(JoinedReceiptParams::new().r_id(1)),
            RequestType::Get,
            Receipt(StoreJoinRow {
                users: vec![StoreUser {
                    name: "Jon".into(),
                    ..Default::default()
                }],
                item_name: "Slamon".into(),
                user_count: 1,
                item_id: 1,
                item_price: 9.49,
                item_qty: 1,
                ..Default::default()
            }),
        ),
        (
            ReqReceipt(JoinedReceiptParams::new().r_id(1).item_qty(3)),
            RequestType::Update,
            Receipt(StoreJoinRow {
                users: vec![StoreUser {
                    name: "Jon".into(),
                    ..Default::default()
                }],
                item_name: "Slamon".into(),
                user_count: 1,
                item_id: 1,
                item_price: 9.49,
                item_qty: 3,
                ..Default::default()
            }),
        ),
        (
            ReqReceipt(JoinedReceiptParams::new().r_id(1)),
            RequestType::Delete,
            DbPayload::AffectedRows(1),
        ),
        (
            ReqReceipt(JoinedReceiptParams::new()),
            RequestType::Reset,
            DbPayload::AffectedRows(0),
        ),
    ];

    for (payload, req_type, want) in test_cases {
        let got = handle_requests(
            DbRequest::new().req_type(req_type).payload(payload),
            &conn,
        )
        .await
        .payload;

        match got {
            Receipt(got) => {
                if let Receipt(want) = want {
                    assert_eq!(
                        want.item_id, got.item_id,
                        "Test if req responds with expected item id"
                    );
                    assert_eq!(
                        want.users.get(0).unwrap().name,
                        got.users.get(0).unwrap().name,
                        "Test if req responds with expected user."
                    );
                    assert_eq!(
                        want.user_count, got.user_count,
                        "Test if req responds w/ expected user_count."
                    );
                    assert_eq!(
                        want.item_qty, got.item_qty,
                        "Test if req responds w/ expected item qty."
                    )
                }
            }
            DbPayload::AffectedRows(_) => {
                assert_eq!(want, got, "Test if req deleted expected rows.")
            }
            _ => panic!(
                "Test expected StoreJoinRow or AffectedRows as DbPayload."
            ),
        }
    }

    Ok(())
}

#[sqlx::test]
async fn test_req_inits(conn: SqlitePool) -> Result<()> {
    // init test
    ItemParams::new()
        .item_name("Slamon")
        .item_price(9.49)
        .post(&conn)
        .await?;
    UserParams::new().user_name("Jon").post(&conn).await?;
    UserParams::new().user_name("Noodle").post(&conn).await?;
    JoinedReceiptParams::new()
        .item_id(1)
        .item_qty(3)
        .add_user(1)
        .add_user(2)
        .post(&conn)
        .await?;

    // test cases
    let test_cases = [
        (ReqUser(UserParams::new()), 2, "users"),
        (ReqItem(ItemParams::new().offset(0)), 1, "items"),
        (
            ReqReceipt(JoinedReceiptParams::new().offset(0)),
            1,
            "receipts",
        ),
    ];

    for (payload, want, desc) in test_cases {
        let res = handle_requests(
            DbRequest::new()
                .payload(payload)
                .req_type(RequestType::GetAll),
            &conn,
        )
        .await;

        if res.error.is_some() {
            let message =
                format!("Test: {desc} failed. {}", res.error.unwrap());
            return Err(Error::msg(message));
        }

        let got = match res.payload {
            DbPayload::Users(u) => u.len(),
            DbPayload::Items(i) => i.len(),
            DbPayload::Receipts(r) => r.len(),
            _ => {
                let req_type = format!("test: {desc}, request type");
                return Err(
                    RequestError::unhandled(req_type, res.payload).into()
                );
            }
        };

        assert_eq!(
            want, got,
            "Test if init handler for {desc} returns expected vec with expected lenght."
        )
    }
    Ok(())
}
