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
