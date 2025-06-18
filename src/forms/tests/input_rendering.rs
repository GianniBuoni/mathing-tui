use super::*;

#[test]
fn test_input_render_block() {
    let input = test_f64_input();
    let mut got = Buffer::empty(test_input_rect());

    let style = Into::<AppStyles>::into(AppColors::INACTIVE);
    let block = input.render_block(style.block_style);

    block.render_ref(got.area, &mut got);
    input
        .render_input(style.input_style)
        .render(block.inner(got.area), &mut got);

    let mut want = Buffer::with_lines(vec![
        "╭ Item Price ────────────────────────────────────╮",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    let block_style = Style::new().fg(Color::DarkGray);
    want.set_style(Rect::new(0, 0, 50, 3), block_style);
    want.set_style(
        Rect::new(1, 1, 48, 1),
        block_style.add_modifier(Modifier::RAPID_BLINK),
    );

    assert_eq!(want, got, "Test input block rendering.");
}

#[test]
fn test_input_render_active_block() {
    let input = test_f64_input();
    let mut got = Buffer::empty(test_input_rect());

    let style = Into::<AppStyles>::into(AppColors::ACTIVE);
    let block = input.render_block(style.block_style);

    block.render_ref(got.area, &mut got);
    input
        .render_input(style.input_style)
        .render(block.inner(got.area), &mut got);
    let mut want = Buffer::with_lines(vec![
        "╭ Item Price ────────────────────────────────────╮",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    let active_input = Style::new()
        .fg(Color::Magenta)
        .add_modifier(Modifier::RAPID_BLINK);

    want.set_style(Rect::new(0, 0, 50, 3), Style::new());
    want.set_style(Rect::new(1, 1, 48, 1), active_input);

    assert_eq!(want, got, "Test active input rendering.")
}
