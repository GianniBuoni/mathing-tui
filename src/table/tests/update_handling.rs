use super::*;

#[test]
fn test_response_handling_items() {
    let mut test_cases = mock_items()
        .into_iter()
        .map(|item| DbResponse::new().payload(DbPayload::Item(item)))
        .collect::<Vec<DbResponse>>();
    let mut mock_receipts = mock_receipts()
        .into_iter()
        .map(|r| DbResponse::new().payload(DbPayload::Receipt(r)))
        .collect::<Vec<DbResponse>>();

    test_cases.append(&mut mock_receipts);

    let mut table = TableData::<StoreItem>::new_builder().build();

    test_cases
        .iter()
        .for_each(|res| table.handle_repsonse(Some(res)));

    assert_eq!(
        table.items.len(),
        3,
        "Test if updating table with a response adds correct items."
    )
}

#[test]
fn test_response_handling_receits() {
    let mut test_cases = mock_items()
        .into_iter()
        .map(|item| DbResponse::new().payload(DbPayload::Item(item)))
        .collect::<Vec<DbResponse>>();
    let mut mock_receipts = mock_receipts()
        .into_iter()
        .map(|r| DbResponse::new().payload(DbPayload::Receipt(r)))
        .collect::<Vec<DbResponse>>();
    test_cases.append(&mut mock_receipts);

    let mut table = TableData::<StoreJoinRow>::new_builder().build();

    test_cases
        .iter()
        .for_each(|res| table.handle_repsonse(Some(res)));

    assert_eq!(
        table.items.len(),
        2,
        "Test if updating table with a response adds correct items"
    )
}
