use std::error::Error;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyModifiers},
};
use tui_input::backend::crossterm::EventHandler;

use super::*;
use crate::test_cases::*;

mod inputs;

#[derive(Default)]
struct EmptyFormData;

fn test_input_area() -> Rect {
    Rect::new(0, 0, 50, 3)
}

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
