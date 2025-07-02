use ratatui::{TerminalOptions, Viewport};

use super::*;

#[test]
fn test_render_block() {
    let colors = Into::<AppStyles>::into(AppColors::ACTIVE);
    let items = TableData::mock_receipts();
    let mut buf = Buffer::empty(TableData::test_rect());

    items
        .render_block(colors.block_style)
        .render(buf.area, &mut buf);

    let mut want = Buffer::with_lines(vec![
        "╭ [0] Receipt Items ─────────────────────────────╮",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "╰──────────────────────────────────────── 0 of 0 ╯",
    ]);

    want.set_style(TableData::test_rect(), Style::default());
    assert_eq!(want, buf, "Test if table renders block widget correctly");
}

#[test]
fn test_render_rows() {
    let items = TableData::mock_items();
    let styles: AppStyles = AppColors::ACTIVE.into();
    let got = items.render_rows(styles.row_style);

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
    let items = TableData::mock_items();
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
    let reciepts = TableData::mock_receipts();
    let receipts = reciepts.render_table(&AppColors::ACTIVE.into());

    let mut state = TableState::new().with_selected(1);
    let mut got = Buffer::empty(TableData::test_rect());

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
    want.set_style(Rect::new(0, 2, 50, 1), Style::new().green().bold());

    assert_eq!(
        want, got,
        "Test if table and table state renders correctly."
    )
}

#[test]
fn test_render_complete_table() -> Result<()> {
    let mut want = Buffer::with_lines(vec![
        "╭ [0] Receipt Items ─────────────────────────────╮",
        "│                                                │",
        "│  Item Name   Item Price  Item Qty   Payees     │",
        "│  Slamon      9.49        1          Jon, Noodl │",
        "│  Blueberrie  5.59        4          Jon        │",
        "│                                                │",
        "│                                                │",
        "╰──────────────────────────────────────── 0 of 0 ╯",
    ]);

    let heading_style = Style::new().fg(Color::Black).bg(Color::Magenta).bold();
    let highlight_style = Style::new().green().bold();

    want.set_style(Rect::new(2, 2, 46, 1), heading_style);
    want.set_style(Rect::new(2, 3, 46, 1), highlight_style);

    let viewport = Viewport::Fixed(TableData::test_rect());
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut term =
        Terminal::with_options(backend, TerminalOptions { viewport })?;
    let mut frame = term.get_frame();

    let mut test_r = TableData::mock_receipts();
    let area = &frame.area();
    test_r.draw(&mut frame, *area);

    let got = frame.buffer_mut().clone();
    assert_eq!(want, got);

    Ok(())
}
