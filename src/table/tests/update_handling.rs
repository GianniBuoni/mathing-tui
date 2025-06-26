use super::*;

#[test]
fn test_response_handling_items() -> Result<()> {
    let mut test_cases = StoreItem::mock()
        .into_iter()
        .map(|item| {
            DbResponse::new()
                .req_type(RequestType::Post)
                .payload(DbPayload::Item(item))
        })
        .collect::<Vec<DbResponse>>();

    let mut mock_receipts = StoreJoinRow::mock()
        .into_iter()
        .map(|r| {
            DbResponse::new()
                .req_type(RequestType::Post)
                .payload(DbPayload::Receipt(r))
        })
        .collect::<Vec<DbResponse>>();

    test_cases.append(&mut mock_receipts);

    let mut table = TableData::builder();
    table.with_table_type(AppArm::Items);
    let mut table = table.build()?;

    test_cases
        .iter()
        .try_for_each(|res| table.handle_response(Some(res)))?;

    assert_eq!(
        3,
        table.items.len(),
        "Test if updating table with a response adds correct items."
    );

    Ok(())
}

#[test]
fn test_response_handling_receits() -> Result<()> {
    let mut test_cases = StoreItem::mock()
        .into_iter()
        .map(|item| {
            DbResponse::new()
                .req_type(RequestType::Post)
                .payload(DbPayload::Item(item))
        })
        .collect::<Vec<DbResponse>>();

    let mut mock_receipts = StoreJoinRow::mock()
        .into_iter()
        .map(|r| {
            DbResponse::new()
                .req_type(RequestType::Post)
                .payload(DbPayload::Receipt(r))
        })
        .collect::<Vec<DbResponse>>();
    test_cases.append(&mut mock_receipts);

    let mut table = TableData::builder();
    table.with_table_type(AppArm::Receipts);
    let mut table = table.build()?;

    test_cases
        .iter()
        .try_for_each(|res| table.handle_response(Some(res)))?;

    assert_eq!(
        2,
        table.items.len(),
        "Test if updating table with a response adds correct items"
    );

    Ok(())
}
