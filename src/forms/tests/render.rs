use ratatui::crossterm::event::KeyEvent;

use super::*;

fn full_form<'a>() -> FormWidget<'a, TestStruct> {
    let item_name = InputWidget::default().title("Item Name");
    let item_price = InputWidget::default().title("Item Price");

    FormWidget::<TestStruct>::new_builder()
        .title("Add New Item")
        .layout([Constraint::Percentage(50), Constraint::Percentage(50)])
        .area(Rect::new(0, 0, 50, 9))
        .add_component(item_name)
        .add_component(item_price)
        .build()
}

fn base_want() -> Buffer {
    Buffer::with_lines(vec![
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
    ])
}

#[test]
fn test_form_render() {
    let form = full_form();
    let mut got = Buffer::empty(test_big_area());
    form.render_ref(got.area, &mut got);

    let mut want = base_want();
    want.set_style(
        Rect::new(4, 3, 48, 3),
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    want.set_style(
        Rect::new(5, 4, 46, 1),
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::RAPID_BLINK),
    );
    want.set_style(
        Rect::new(4, 6, 48, 3),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::RAPID_BLINK),
    );
    assert_eq!(
        want, got,
        "Test if builder methods render expected form and activate first feild."
    );
}

#[test]
fn test_form_next_feild_styles() {
    let mut form = full_form();
    form.next_feild();

    let mut got = Buffer::empty(test_big_area());
    form.render_ref(got.area, &mut got);

    let mut want = base_want();

    let inactive_style = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::RAPID_BLINK);
    let active_border_style =
        Style::default().add_modifier(Modifier::RAPID_BLINK);
    let active_style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::RAPID_BLINK);

    want.set_style(Rect::new(4, 3, 48, 3), inactive_style);
    want.set_style(Rect::new(4, 6, 48, 3), active_border_style);
    want.set_style(Rect::new(5, 7, 46, 1), active_style);

    assert_eq!(
        want, got,
        "Test if `next_field` method properly advances styles"
    );
}

#[test]
fn test_form_next_feild_cursors() {
    let mut form = full_form();
    form.next_feild();

    // render is required to set cursor position
    let mut buf = Buffer::empty(test_big_area());
    form.render_ref(buf.area, &mut buf);

    let got = form.cursor_pos().unwrap();
    let want = Position::new(5, 7);

    assert_eq!(want, got, "Test if input gives correct cursor potision.");
}

#[test]
fn test_form_prev_feild() {
    let mut form = full_form();
    form.prev_feild();

    // render is required to set cursor position
    let mut buf = Buffer::empty(test_big_area());
    form.render_ref(buf.area, &mut buf);

    let got = form.cursor_pos().unwrap();
    // since there are onlys two it should wrap back to the last
    let want = Position::new(5, 7);

    assert_eq!(want, got, "Test if input gives correct cursor potision.");
}

#[test]
fn test_form_handle_inputs_tracking_cursor() {
    let mut form = full_form();
    let mut buf = Buffer::empty(test_big_area());

    let test_cases = [
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE),
            Position::new(5, 7),
            "Test tab go to next feild.",
        ),
        (
            KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            Position::new(6, 7),
            "Test handling char input",
        ),
        (
            KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT),
            Position::new(5, 4),
            "Test handling SHIFT+Tab",
        ),
    ];

    test_cases.iter().for_each(|(key_event, want, desc)| {
        form.handle_event(key_event);
        form.render(buf.area, &mut buf);

        let got = form.cursor_pos().unwrap();
        assert_eq!(got, *want, "{desc}");
    });
}
