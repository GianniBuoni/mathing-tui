use ratatui::{TerminalOptions, Viewport};

use super::*;

#[test]
fn test_form_render_block() {
    let form = test_form();
    let mut got = Buffer::empty(test_big_rect());
    let [_, _, form_area, _] = form.get_block_areas(got.area);
    form.render_block(form_area, &mut got);

    let want = Buffer::with_lines(vec![
        "                                                        ",
        "                                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   ╰────────────────────────────────────────────────╯   ",
        "                                                        ",
        "                                                        ",
    ]);

    assert_eq!(want, got, "Test form block rendering.");
}

#[test]
fn test_form_render() -> Result<()> {
    let mut form = test_valid_form();
    form.mut_inner(|f| f.map_err(Some(FormErrors::malformed("fields").into())));

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
        "    Malformed: form has no fields.                      ",
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
    want.set_style(Rect::new(3, 10, 50, 1), Style::new().fg(Color::Red));

    assert_eq!(want, got, "Test form draw method.");
    Ok(())
}
