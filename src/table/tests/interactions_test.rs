use super::*;

#[test]
fn test_table_constuction() {
    let items = TableData::mock_items();
    let desc = "Test table.active data is constructed correctly";

    assert!(items.is_active(), "{desc}");
    assert_eq!(items.app_index, 0, "{desc}");
    assert_eq!(items.table_index, 0, "{desc}")
}

#[test]
fn test_row_increment() {
    let mut items = TableData::mock_items();

    for i in 0..3 {
        let want = match i {
            2 => 0,
            _ => i + 1,
        };
        let desc = format!(
            "Test incrementing table row. Table has {} items; row index should be {want} (i + 1)",
            items.items.len()
        );

        items.row_increment(1);
        assert_eq!(want, items.table_index, "{desc}")
    }
}

#[test]
fn test_row_decrement() {
    let mut items = TableData::mock_items();

    for i in 0..3 {
        let want = 2 - i;
        let desc = format!(
            "test decrementing table row. Table has {} items; row index should be {want} (2 - i)",
            items.items.len()
        );

        items.row_increment(-1);
        assert_eq!(want, items.table_index, "{desc}")
    }
}

#[test]
fn test_up_down_navigation_input() {
    let test_cases = [
        (Action::NavigateUp, 2, "Test up action from index 0."),
        (Action::NavigateUp, 1, "Test up action from index 2."),
        (Action::NavigateDown, 2, "Test down action from index 1."),
        (Action::NavigateDown, 0, "Test down action from index 2."),
    ];

    let mut table = TableData::mock_items();

    test_cases.into_iter().for_each(|(action, want, desc)| {
        table.handle_action(Some(action));
        assert_eq!(want, table.table_index, "{desc}");
    });
}

#[test]
fn test_max_pages() {
    let desc = "Test max page math calculations for tables:";
    let test_cases = [(3_i64, "55 / 20"), (2, "40 / 20"), (1, "5 / 13 ")];
    test_cases
        .into_iter()
        .zip(TableData::mock_pages())
        .for_each(|((want, test), table)| {
            assert_eq!(want, table.max_pages(), "{desc} {test}.");
        });
}

#[test]
fn test_req_offset() {
    let desc = "Test request offset calculation for tables:";
    let test_cases = [
        (40_i64, "55 items, page 3, limit 20"),
        (20, "40 items, page 2, limit 20"),
        (0, "5 items, page 1, limit 13"),
    ];
    test_cases
        .into_iter()
        .zip(TableData::mock_pages())
        .for_each(|((want, test), table)| {
            assert_eq!(want, table.get_req_offset(), "{desc} {test}.")
        });
}
