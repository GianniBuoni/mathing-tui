use std::rc::Rc;

use super::*;

const PAGING_ITEMS: [&str; 40] = [
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
    "Tangerine",
    "Watermelon",
    "Apricot",
    "Blueberry",
    "Cantaloupe",
    "Guava",
    "Lychee",
    "Dragonfruit",
    "Coconut",
    "Persimmon",
    "Passionfruit",
    "Cranberry",
    "Jackfruit",
    "Starfruit",
    "Soursop",
    "Mulberry",
    "Açaí",
    "Chayote",
    "Clementine",
    "Gooseberry",
];

const PAGING_PRICES: [f64; 40] = [
    1.99, 2.49, 3.00, 4.75, 5.99, 6.49, 7.25, 8.99, 9.99, 10.50, 11.75, 12.00,
    13.99, 14.25, 15.50, 16.00, 17.95, 18.25, 19.99, 20.00, 21.50, 22.99,
    23.75, 24.99, 25.50, 26.00, 27.99, 28.25, 29.99, 30.00, 31.50, 32.75,
    33.99, 34.00, 35.99, 36.50, 37.75, 38.99, 39.00, 40.00,
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

    Ok(())
}

pub async fn try_init_paging_test(conn: &SqlitePool) -> Result<TableData> {
    try_init_paging_db(conn).await?;
    let mut table = TableData::builder();
    table
        .with_title("Paging Items")
        .with_heading("Item Name")
        .with_heading("Item Price")
        .with_table_type(AppArm::Items);
    let mut table = table.build()?;

    let counts = ItemParams::default().count(conn).await?;
    let res = ItemParams::default().get_all(conn).await?;

    vec![
        DbResponse::new()
            .req_type(RequestType::Count)
            .payload(DbPayload::Count(AppArm::Items, counts)),
        DbResponse::new()
            .req_type(RequestType::GetAll)
            .payload(DbPayload::Items(res)),
    ]
    .iter()
    .try_for_each(|f| table.handle_response(Some(f)))?;

    assert_eq!(1, table.current_page, "Table current page init");
    assert_eq!(40, table.count, "Table count init");

    Ok(table)
}

/// Takes the DbRequest, collects the cascading requests,
/// runs all the requests through the handler, and sends
/// the responses back to the table.
pub async fn try_process_req(
    conn: &SqlitePool,
    table: &mut TableData,
    req: DbRequest,
) -> Result<()> {
    let mut table_req = TryInto::<TableReq>::try_into(&req)?;
    table_req.push(req);
    table.collect_reqs(&mut table_req);

    for req in table_req.reqs {
        let res = handle_requests(req, conn).await;
        table.handle_response(Some(&res))?;
    }
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
