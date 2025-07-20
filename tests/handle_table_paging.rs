use common::prelude::*;

mod common;

fn test_table() -> Result<TableData> {
    let mut table = TableData::builder();
    table
        .with_title("Paging Items")
        .with_heading("Item Name")
        .with_heading("Item Price")
        .with_table_type(AppArm::Items);
    table.build()
}

#[sqlx::test]
fn test_post_paging(conn: SqlitePool) -> Result<()> {
    try_init_paging_db(&conn).await?;

    let mut table = test_table()?;

    let counts = ItemParams::default().count(&conn).await?;
    let res = ItemParams::default().get_all(&conn).await?;
    let res = [
        DbResponse::new()
            .req_type(RequestType::Count)
            .payload(DbPayload::Count(AppArm::Items, counts)),
        DbResponse::new()
            .req_type(RequestType::GetAll)
            .payload(DbPayload::Items(res)),
    ];

    res.iter()
        .try_for_each(|f| table.handle_response(Some(f)))?;

    table.handle_action(Some(Action::NavigateRight));
    assert_eq!(2, table.pages, "Test basic paging.");

    table.handle_action(Some(Action::NavigateRight));
    assert_eq!(1, table.pages, "Test paging back.");

    let res = DbResponse::new()
        .req_type(RequestType::Post)
        .payload(DbPayload::Item(StoreItem::default()));

    table.handle_response(Some(&res))?;

    assert_eq!(
        2, table.pages,
        "Test post response auto paging to last page."
    );

    Ok(())
}
