use super::*;

#[test]
fn test_render_block() {
    let colors = AppColors::ACTIVE;
    let items = mock_receipts();
    let mut buf = Buffer::empty(test_rect());

    items
        .render_block(&colors.border_fg)
        .render(buf.area, &mut buf);

    let mut want = Buffer::with_lines(vec![
        "╭ [0] Receipt Items ─────────────────────────────╮",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    want.set_style(test_rect(), Style::default());
    assert_eq!(want, buf, "Test if table renders block widget correctly");
}

#[test]
fn test_render_rows() {
    let items = mock_items();
    let got = items.render_rows(&AppColors::ACTIVE.into());

    // NOTE: On its own, the first row does not have the highlght styles set
    // its up to the [`render_table`] function to correctly set the highlght.
    let want = Row::new([
        Cell::new(Text::from(Line::from(" Slamon "))),
        Cell::new(Text::from(Line::from(" 9.49 "))),
    ])
    .style(Style::default().fg(Color::Reset));

    assert_eq!(3, got.len(), "Test if correct amount of rows are returned.");
    assert_eq!(want, got[0], "Test if table outputs expected row.")
}

#[test]
fn test_render_heading() {
    let items = mock_items();
    let got = items.render_heading(&AppColors::ACTIVE.into());

    let want = Row::new([
        Cell::new(Text::from(Line::from(" Items "))),
        Cell::new(Text::from(Line::from(" Price "))),
    ])
    .style(Style::new().black().on_magenta().bold());

    assert_eq!(want, got, "Test if table outputs expected heading row");
}

#[test]
fn test_render_table() {
    let reciepts = mock_receipts();
    let receipts = reciepts.render_table(&AppColors::ACTIVE.into());

    let mut state = TableState::new().with_selected(1);
    let mut got = Buffer::empty(test_rect());

    {
        use ratatui::widgets::StatefulWidget;
        StatefulWidget::render(&receipts, got.area, &mut got, &mut state);
    }

    let mut want = Buffer::with_lines(vec![
        " Item Name    Item Price   Item Qty    Payees     ",
        " Slamon       9.49         1           Jon, Noodle",
        " Blueberries  5.59         4           Jon        ",
        "                                                  ",
        "                                                  ",
        "                                                  ",
        "                                                  ",
        "                                                  ",
    ]);

    want.set_style(
        Rect::new(0, 0, 50, 1),
        Style::new().black().on_magenta().bold(),
    );
    want.set_style(Rect::new(0, 1, 50, 1), Style::new().fg(Color::Reset));
    want.set_style(Rect::new(0, 2, 50, 1), Style::new().red());

    assert_eq!(
        want, got,
        "Test if table and table state renders correctly."
    )
}
