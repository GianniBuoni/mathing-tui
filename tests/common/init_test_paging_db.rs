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

pub async fn try_init_paging_test(conn: &SqlitePool) -> Result<[TableData; 2]> {
    try_init_paging_db(conn).await?;

    let mut item_table = TableData::builder();
    item_table.with_table_type(AppArm::Items).with_item_limit(4);
    let item_table = item_table.build()?;

    let mut receipt_table = TableData::builder();
    receipt_table
        .with_table_type(AppArm::Receipts)
        .with_item_limit(4);
    let receipt_table = receipt_table.build()?;

    let mut tables = [item_table, receipt_table];
    for table in tables.iter_mut() {
        let get = handle_requests(table.get_req().unwrap(), conn).await;
        let count = handle_requests(table.count().unwrap(), conn).await;
        table.handle_response(Some(&get))?;
        table.handle_response(Some(&count))?;
    }

    {
        let [i, r] = &tables;
        assert_eq!(1, i.current_page, "Item table current page init");
        assert_eq!(20, i.count, "Item table count init");
        assert_eq!(4, i.get_items().len(), "Item table length init.");
        assert_eq!(1, r.current_page, "Receipt table current page init");
        assert_eq!(8, r.count, "Receipt table count init");
        assert_eq!(4, r.get_items().len(), "Receipt table length init");
    }

    Ok(tables)
}

/// Takes the DbRequest, collects the cascading requests,
/// runs all the requests through the handler, and sends
/// the responses back to the table.
pub async fn try_process_req(
    conn: &SqlitePool,
    tables: &mut [TableData; 2],
    req: DbRequest,
) -> Result<()> {
    let mut table_req = TryInto::<TableReq>::try_into(req)?;
    tables
        .iter_mut()
        .for_each(|f| f.collect_reqs(&mut table_req));

    for req in table_req.reqs {
        dbg!(&req);
        let res = handle_requests(req, conn).await;
        tables
            .iter_mut()
            .try_for_each(|f| f.handle_response(Some(&res)))?;
    }
    Ok(())
}

/// Returns a request for the test's corresponding DB operation.
/// By default returns a `Refresh` request to refetch all table items
/// removing any filters and going back to the first page.
pub fn test_item_req(req_type: RequestType) -> DbRequest {
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
                .with_item_id(ParamOption::new().map_value(3).to_owned())
                .with_item_name(
                    ParamOption::new().map_value("Updated Item").to_owned(),
                )
                .build(),
        ),
        RequestType::Delete => DbPayload::ItemParams(
            ItemParams::builder()
                .with_item_id(ParamOption::new().map_value(5).to_owned())
                .build(),
        ),
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

pub fn test_r_req(req_type: RequestType) -> DbRequest {
    let payload = match req_type {
        RequestType::Post => DbPayload::ReceiptParams(
            JoinedReceiptParams::builder()
                .with_item_id(ParamOption::new().map_value(1).to_owned())
                .with_item_qty(ParamOption::new().map_value(1).to_owned())
                .with_user(1)
                .build(),
        ),
        RequestType::Update => DbPayload::ReceiptParams(
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(5).to_owned())
                .with_item_qty(ParamOption::new().map_value(1).to_owned())
                .build(),
        ),
        RequestType::Delete => DbPayload::ReceiptParams(
            JoinedReceiptParams::builder()
                .with_r_id(ParamOption::new().map_value(5).to_owned())
                .build(),
        ),
        RequestType::Reset => {
            DbPayload::ReceiptParams(JoinedReceiptParams::default())
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
/// arbitrarily sets the Items test table to page page passed in.
pub async fn page_items_to(
    tables: &mut [TableData; 2],
    conn: &SqlitePool,
    page: i64,
) -> Result<()> {
    let i = tables.first_mut().unwrap();
    i.next_page = page;

    let i = handle_requests(i.get_req().unwrap(), conn).await;
    tables
        .iter_mut()
        .try_for_each(|f| f.handle_response(Some(&i)))?;

    Ok(())
}
/// arbitrarily sets the test table to page page passed in.
/// used for tests that need to check if a req/res will
/// cause table to go back to the first page.
pub async fn page_to(
    tables: &mut [TableData; 2],
    conn: &SqlitePool,
    page: i64,
) -> Result<()> {
    tables.iter_mut().for_each(|f| {
        f.next_page = page;
    });

    let [i, r] = &tables;
    let i = handle_requests(i.get_req().unwrap(), conn).await;
    let r = handle_requests(r.get_req().unwrap(), conn).await;

    tables.iter_mut().try_for_each(|f| {
        f.handle_response(Some(&i))?;
        f.handle_response(Some(&r))
    })?;

    Ok(())
}
