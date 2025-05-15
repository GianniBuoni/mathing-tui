use std::error::Error;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyModifiers},
};
use tui_input::backend::crossterm::EventHandler;

use super::*;
use crate::test_cases::*;

mod inputs;
mod render;

fn test_big_area() -> Rect {
    Rect::new(0, 0, 56, 10)
}

fn test_input_area() -> Rect {
    Rect::new(0, 0, 50, 3)
}

#[derive(Default, Debug)]
struct TestStruct;

#[test]
fn test_form() {
    let form: FormWidget<TestStruct> = FormWidget::default().area(test_rect());
    let mut got = Buffer::empty(test_big_area());

    form.render_ref(got.area, &mut got);

    let want = Buffer::with_lines(vec![
        "                                                        ",
        "                                                        ",
        "   ╭────────────────────────────────────────────────╮   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   │                                                │   ",
        "   ╰────────────────────────────────────────────────╯   ",
        "                                                        ",
    ]);

    assert_eq!(want, got)
}

#[test]
fn test_form_menu() {
    let line = "Add New Item";
    let form: FormWidget<TestStruct> =
        FormWidget::default().area(test_rect()).title(line);
    let mut got = Buffer::empty(test_big_area());

    form.render_ref(got.area, &mut got);

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

    assert_eq!(want, got)
}
