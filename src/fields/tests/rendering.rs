use ratatui::{TerminalOptions, Viewport};

use super::*;

#[test]
fn test_input_render_block() {
    let input = InputField::test_item_price();
    let mut got = Buffer::empty(InputField::test_rect());

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
    let input = InputField::test_item_price();
    let mut got = Buffer::empty(InputField::test_rect());

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
fn test_choices() {
    let mut test_choice = Choice::<i64>::test_choice_1();

    let test_cases = [
        (
            Line::from("  ◉︎ Jon").dark_gray().bold(),
            1_usize,
            true,
            "Test inactive selected choice",
        ),
        (
            Line::from("  ○︎ Jon").dark_gray().bold(),
            1,
            false,
            "Test inactive unselected choice.",
        ),
        (
            Line::from("> ◉︎ Jon").green().bold(),
            0,
            true,
            "Test active selected choice",
        ),
        (
            Line::from("> ○︎ Jon").green().bold(),
            0,
            false,
            "Test active unselected choice.",
        ),
    ];

    test_cases
        .into_iter()
        .for_each(|(want, active, selected, desc)| {
            test_choice.active_choice.go_to(active);
            test_choice.selected = selected;

            let mut got_buffer = Buffer::empty(Choice::<bool>::test_rect());
            let got = test_choice.get_display();
            got.render(got_buffer.area, &mut got_buffer);

            let mut want_buffer = Buffer::empty(Choice::<bool>::test_rect());
            want.render(want_buffer.area, &mut want_buffer);

            assert_eq!(want_buffer, got_buffer, "{desc}");
        });
}

#[test]
fn test_selection_fields() -> Result<()> {
    let mut test_case = SelectionField::mock();

    // set up terminal
    let viewport = Viewport::Fixed(SelectionField::test_rect());
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut term =
        Terminal::with_options(backend, TerminalOptions { viewport })?;
    let mut frame = term.get_frame();

    test_case.draw(&mut frame, SelectionField::test_rect());
    let got = frame.buffer_mut().clone();

    let mut want = Buffer::with_lines([
        "╭ Add Users ─────────────────────────────────────╮",
        "│                                                │",
        "│ > ○︎ Jon                                        │",
        "│   ○︎ Noodle                                     │",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    want.set_style(Rect::new(2, 2, 46, 1), Style::default().green().bold());
    want.set_style(Rect::new(2, 3, 46, 1), Style::default().dark_gray().bold());

    assert_eq!(want, got, "Test selection field rendering");
    Ok(())
}
