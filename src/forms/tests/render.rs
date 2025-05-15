use super::*;

#[test]
fn test_form_render() {
    let item_name = InputWidget::default().title("Item Name");
    let item_price = InputWidget::default().title("Item Price");

    let form: FormWidget<TestStruct> = FormWidget::default()
        .title("Add New Item")
        .layout([Constraint::Percentage(50), Constraint::Percentage(50)])
        .area(Rect::new(0, 0, 50, 9))
        .add_component(item_name)
        .add_component(item_price);

    let mut got = Buffer::empty(test_big_area());
    form.render_ref(got.area, &mut got);

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
    ]);

    want.set_style(
        Rect::new(4, 3, 48, 6),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::RAPID_BLINK),
    );

    assert_eq!(want, got);
}
