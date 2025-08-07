use std::rc::Rc;

use super::*;

const PAGING_ITEMS: [&str; 20] = [
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew",
    "Kiwi",
    "Lemon",
    "Mango",
    "Nectarine",
    "Orange",
    "Papaya",
    "Pear",
    "Pineapple",
    "Plum",
    "Pomegranate",
    "Raspberry",
    "Strawberry",
];

const PAGING_PRICES: [f64; 20] = [
    1.99, 2.49, 3.00, 4.75, 5.99, 6.49, 7.25, 8.99, 9.99, 10.50, 11.75, 12.00,
    13.99, 14.25, 15.50, 16.00, 17.95, 18.25, 19.99, 20.00,
];

const PAGING_RECEIPTS: [(i64, i64); 8] = [
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 8),
    (8, 9),
];

const PAGING_USERS: [&str; 2] = ["Noodle", "Blue"];

const PAGING_RU: [(i64, i64); 8] = [
    (1, 1),
    (2, 2),
    (3, 1),
    (4, 2),
    (5, 1),
    (6, 2),
    (7, 1),
    (8, 2),
];

async fn try_init_paging_db(conn: &SqlitePool) -> Result<()> {
    let now = AppConfig::try_get_time()?;
    let items = PAGING_ITEMS
        .into_iter()
        .zip(PAGING_PRICES)
        .collect::<Rc<[(&str, f64)]>>();

    // add items
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO items (name, price, created_at, updated_at) ",
    );
    q.push_values(items.iter(), |mut q, (name, price)| {
        q.push_bind(name)
            .push_bind(price)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add users
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO users (name, created_at, updated_at) ",
    );
    q.push_values(PAGING_USERS, |mut q, name| {
        q.push_bind(name).push_bind(now).push_bind(now);
    });
    q.build().execute(conn).await?;

    // add receipts
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT INTO receipts (item_id, item_qty, created_at, updated_at) ",
    );
    q.push_values(PAGING_RECEIPTS, |mut q, (item_id, item_qty)| {
        q.push_bind(item_id)
            .push_bind(item_qty)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    // add ru
    let mut q = QueryBuilder::<Sqlite>::new(
        "INSERT into receipts_users (receipt_id, user_id, created_at, updated_at) ",
    );
    q.push_values(PAGING_RU, |mut q, (r_id, u_id)| {
        q.push_bind(r_id)
            .push_bind(u_id)
            .push_bind(now)
            .push_bind(now);
    });
    q.build().execute(conn).await?;

    Ok(())
}

pub async fn try_init_paging_test(conn: &SqlitePool) -> Result<Vec<TableData>> {
    try_init_paging_db(conn).await?;

    let mut item_table = TableData::builder();
    item_table.with_table_type(AppArm::Items).with_item_limit(4);
    let mut item_table = item_table.build()?;

    let mut receipt_table = TableData::builder();
    receipt_table
        .with_table_type(AppArm::Receipts)
        .with_item_limit(4);
    let mut receipt_table = receipt_table.build()?;

    // initialize both tables
    let item_counts = ItemParams::default().count(conn).await?;
    let item_res = ItemParams::default().get_all(conn).await?;

    [
        DbResponse::new()
            .req_type(RequestType::Count)
            .payload(DbPayload::Count(AppArm::Items, item_counts)),
        DbResponse::new()
            .req_type(RequestType::GetAll)
            .payload(DbPayload::Items(item_res)),
    ]
    .iter()
    .try_for_each(|f| item_table.handle_response(Some(f)))?;

    let receipt_counts = JoinedReceiptParams::default().count(conn).await?;
    let receipt_res = JoinedReceiptParams::default().get_all(conn).await?;

    [
        DbResponse::new()
            .req_type(RequestType::Count)
            .payload(DbPayload::Count(AppArm::Receipts, receipt_counts)),
        DbResponse::new()
            .req_type(RequestType::GetAll)
            .payload(DbPayload::Receipts(receipt_res)),
    ]
    .iter()
    .try_for_each(|f| receipt_table.handle_response(Some(f)))?;

    assert_eq!(1, item_table.current_page, "Item table current page init");
    assert_eq!(20, item_table.count, "Item table count init");
    assert_eq!(
        1, receipt_table.current_page,
        "Receipt table current page init"
    );
    assert_eq!(8, receipt_table.count, "Receipt table count init");

    Ok(vec![item_table, receipt_table])
}

/// Takes the DbRequest, collects the cascading requests,
/// runs all the requests through the handler, and sends
/// the responses back to the table.
pub async fn try_process_req(
    conn: &SqlitePool,
    tables: &mut Vec<TableData>,
    req: DbRequest,
) -> Result<()> {
    let mut table_req = TryInto::<TableReq>::try_into(req)?;
    tables.first_mut().unwrap().collect_reqs(&mut table_req);

    let results = futures::future::join_all(
        table_req
            .reqs
            .into_iter()
            .map(async |req| handle_requests(req, conn).await),
    )
    .await;

    tables.iter_mut().try_for_each(|f| {
        results.iter().try_for_each(|g| f.handle_response(Some(g)))
    })?;

    Ok(())
}

/// Returns a request for the test's corresponding DB operation.
/// By default returns a `Refresh` request to refetch all table items
/// removing any filters and going back to the first page.
pub fn test_req(req_type: RequestType) -> DbRequest {
    let payload = match req_type {
        RequestType::Post => DbPayload::ItemParams(
            ItemParams::builder()
                .with_item_name(
                    ParamOption::new().map_value("New item").to_owned(),
                )
                .with_item_price(ParamOption::new().map_value(1.).to_owned())
                .build(),
        ),
        RequestType::Update => DbPayload::ItemParams(
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(21).to_owned())
                .with_item_name(
                    ParamOption::new().map_value("Updated Item").to_owned(),
                )
                .build(),
        ),
        RequestType::Delete => DbPayload::ItemParams(
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(40).to_owned())
                .build(),
        ),
        RequestType::Reset => {
            DbPayload::ReceiptParams(JoinedReceiptParams::default())
        }
        RequestType::GetAll => {
            DbPayload::ItemParams(ItemParams::default().with_search("e"))
        }
        _ => {
            return DbRequest::new()
                .with_req_type(RequestType::Update)
                .with_payload(DbPayload::StoreTotal);
        }
    };

    DbRequest::new()
        .with_req_type(req_type)
        .with_payload(payload)
}

/// generates a get req from the table; the passed in action is handled
/// defore any requests are made.
pub fn basic_get_req(
    tables: &mut Vec<TableData>,
    action: Option<Action>,
) -> Result<DbRequest> {
    let items = tables.first_mut().unwrap();
    items.handle_action(action);

    let message = "Couldn't get a set of paging requests.";
    items.get_req().ok_or_else(|| Error::msg(message))
}
