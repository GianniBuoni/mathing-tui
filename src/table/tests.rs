use super::*;
use crate::test_cases::*;

#[test]
fn test_table_constuction() {
    let items = mock_items();
    let desc = "Test table.active data is constructed correctly";

    assert!(!items.is_active(), "{desc}");
    assert_eq!(items.app_index, 0, "{desc}");
    assert_eq!(items.table_index, 0, "{desc}")
}

#[test]
fn test_row_increment() {
    let mut items = mock_items();

    for i in 0..3 {
        let want = match i {
            2 => 0,
            _ => i + 1,
        };
        let desc = format!(
            "Test incrementing table row. Table has {} items; row index should be {want} (i + 1)",
            items.items.len()
        );

        items.next_row();
        assert_eq!(want, items.table_index, "{desc}")
    }
}

#[test]
fn test_row_decrement() {
    let mut items = mock_items();

    for i in 0..3 {
        let want = 2 - i;
        let desc = format!(
            "test decrementing table row. Table has {} items; row index should be {want} (2 - i)",
            items.items.len()
        );

        items.prev_row();
        assert_eq!(want, items.table_index, "{desc}")
    }
}
