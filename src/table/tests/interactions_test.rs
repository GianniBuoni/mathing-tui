use super::*;

#[test]
fn test_table_constuction() {
    let items = mock_items();
    let desc = "Test table.active data is constructed correctly";

    assert!(!items.active, "{desc}");
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

#[test]
fn test_down_navigation_input() {
    let key_codes = [
        (KeyCode::Char('j'), "Test j char input"),
        (KeyCode::Down, "Test down key input"),
    ];

    key_codes.into_iter().for_each(|(key, desc)| {
        let mut items = mock_items();
        let action = items.handle_events(Some(Event::Key(KeyEvent::from(key))));
        items.update(action);

        assert_eq!(items.table_index.clone(), 1, "{desc}");
    });
}
