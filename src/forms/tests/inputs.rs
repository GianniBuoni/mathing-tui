use super::*;

#[test]
fn test_input_widget() {
    let input = InputWidget::default().title("Add Items");
    let mut got = Buffer::empty(test_input_area());

    input.render_ref(got.area, &mut got);

    let mut want = Buffer::with_lines(vec![
        "╭ Add Items ─────────────────────────────────────╮",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    want.set_style(
        Rect::new(0, 0, 50, 8),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::RAPID_BLINK),
    );
    assert_eq!(want, got, "Test inactive input component.")
}

#[test]
fn test_input_toggle_style() {
    let mut input = InputWidget::default().title("Active Input");
    input.toggle();
    let mut got = Buffer::empty(test_input_area());

    input.render_ref(got.area, &mut got);

    let mut want = Buffer::with_lines(vec![
        "╭ Active Input ──────────────────────────────────╮",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
    ]);

    let block_style = Style::default().add_modifier(Modifier::RAPID_BLINK);
    let input_style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::RAPID_BLINK);

    want.set_style(Rect::new(0, 0, 50, 3), block_style);
    want.set_style(Rect::new(1, 1, 48, 1), input_style);

    assert_eq!(want, got, "Test inactive input component.")
}

#[test]
fn test_input_get_cursor() -> Result<(), Box<dyn Error>> {
    let mut input = InputWidget::default().title("Active Input");
    input.toggle();
    let mut buf = Buffer::empty(test_input_area());
    input.render_ref(buf.area, &mut buf);

    let want = Position::new(1, 1);
    let got = input.get_cursor()?;

    assert_eq!(
        want, got,
        "Test if toggling input active creates non-default cursor position."
    );

    Ok(())
}

#[test]
fn test_input_handle_event() -> Result<(), Box<dyn Error>> {
    // init input
    let mut input = InputWidget::default().title("Active Input");
    input.toggle();
    let mut buf = Buffer::empty(test_input_area());

    // input events
    let events = [
        event::Event::Key(event::KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
        )),
        event::Event::Key(event::KeyEvent::new(
            KeyCode::Char('b'),
            KeyModifiers::NONE,
        )),
        event::Event::Key(event::KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::NONE,
        )),
    ];
    events.iter().for_each(|event| {
        input.input.handle_event(event);
    });
    input.render_ref(buf.area, &mut buf);

    // process side effects
    let want = Position::new(4, 1);
    let got = input.get_cursor()?;

    assert_eq!(want, got);

    Ok(())
}
