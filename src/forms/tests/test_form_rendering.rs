use ratatui::{TerminalOptions, Viewport};

use super::*;

#[test]
fn test_form_render_block() {
    let form = test_form();
    let mut got = Buffer::empty(test_big_rect());

    let blocks = form.render_block();
    let areas = form.render_block_areas(blocks.first().unwrap(), got.area);

    blocks
        .iter()
        .zip(areas.iter())
        .for_each(|(block, area)| block.render(*area, &mut got));

    let want = Buffer::with_lines(vec![
        "                                                        ",
        "    Add New Item                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │                                                │   ",
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

#[test]
fn test_form_render() -> Result<()> {
    let mut form = test_full_form();

    // set up terminal
    let viewport = Viewport::Fixed(test_big_rect());
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut term =
        Terminal::with_options(backend, TerminalOptions { viewport })?;
    let mut frame = term.get_frame();

    form.draw(&mut frame, test_big_rect());
    let got = frame.buffer_mut().clone();

    let mut want = Buffer::with_lines(vec![
        "                                                        ",
        "    Add New Item                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │╭ Item Name ───────────────────────────────────╮│   ",
        "   ││                                              ││   ",
        "   │╰──────────────────────────────────────────────╯│   ",
        "   │╭ Item Price ──────────────────────────────────╮│   ",
        "   ││                                              ││   ",
        "   │╰──────────────────────────────────────────────╯│   ",
        "   ╰────────────────────────────────────────────────╯   ",
        "                                                        ",
    ]);

    let active_input = Style::new()
        .fg(Color::Magenta)
        .add_modifier(Modifier::RAPID_BLINK);
    let inactive_block = Style::new().fg(Color::DarkGray);

    want.set_style(Rect::new(5, 4, 46, 1), active_input);
    want.set_style(Rect::new(4, 6, 48, 3), inactive_block);
    want.set_style(
        Rect::new(5, 7, 46, 1),
        inactive_block.add_modifier(Modifier::RAPID_BLINK),
    );

    assert_eq!(want, got, "Test form draw method.");
    Ok(())
}
