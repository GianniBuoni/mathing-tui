use ratatui::{buffer::Buffer, widgets::WidgetRef};

use crate::test_cases::*;

use super::*;

#[derive(Default)]
struct EmptyFormData;

#[test]
fn test_defaut_form() {
    let form: FormWidget<EmptyFormData> = FormWidget::default();
    let mut got = Buffer::empty(test_rect());

    form.render_ref(got.area, &mut got);

    let want = Buffer::with_lines(vec![
        "╭────────────────────────────────────────────────╮",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
        "                                                  ",
    ]);

    assert_eq!(want, got)
}

#[test]
fn test_form_menu() {
    let line = "Submit <enter> | Cancel <esc>";
    let form: FormWidget<MockReceipt> = FormWidget::default().menu(line);
    let mut got = Buffer::empty(test_rect());

    form.render_ref(got.area, &mut got);

    let want = Buffer::with_lines(vec![
        "╭────────────────────────────────────────────────╮",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "│                                                │",
        "╰────────────────────────────────────────────────╯",
        "          Submit <enter> | Cancel <esc>           ",
    ]);

    assert_eq!(want, got)
}

#[test]
fn test_registering_forms() {}
