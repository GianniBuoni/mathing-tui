use super::*;

#[test]
fn test_form_render_block() {
    let form = test_form();
    let mut got = Buffer::empty(test_big_rect());

    form.render_block(got.area, &mut got);

    let want = Buffer::with_lines(vec![
        "                                                        ",
        "    Add New Item                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   ╰────────────────────────────────────────────────╯   ",
        "                                                        ",
    ]);

    assert_eq!(want, got, "Test form block rendering.");
}

#[test]
fn test_input_render_block() {
    let input = test_input();
    let mut got = Buffer::empty(test_input_rect());
    input.render_block(got.area, &mut got);

    let mut want = Buffer::with_lines(vec![
        "╭ Item Name ─────────────────────────────────────╮",
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

fn test_input_render_active_block() {
    let mut input = test_input();
    input.active = true;

    let mut got = Buffer::empty(test_input_rect());
    input.render_block(got.area, &mut got);

    let mut want = Buffer::with_lines(vec![
        "╭ Item Name ─────────────────────────────────────╮",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    let active_block = Style::new().add_modifier(Modifier::RAPID_BLINK);
    let active_input = Style::new()
        .fg(Color::Magenta)
        .add_modifier(Modifier::RAPID_BLINK);

    want.set_style(Rect::new(0, 0, 50, 3), active_block);
    want.set_style(Rect::new(1, 1, 48, 1), active_input);

    assert_eq!(want, got, "Test active input rendering.")
}
