#![allow(dead_code)]
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use tui_input::Input;

use crate::prelude::*;

mod builder;
mod form_data;
mod form_field;
mod form_tui;
mod form_value;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{Form, FormField, FormTui};
}

#[derive(Debug)]
pub enum FormTui<'a> {
    ItemForm(Form<'a>),
    ReceiptForm(Form<'a>),
}

#[derive(Debug)]
pub enum FormValue<'a> {
    String(Cow<'a, str>),
    Decimal(f64),
    Integer(i64),
}

#[derive(Default, Debug)]
pub struct FormField<'a> {
    title: Cow<'a, str>,
    index: usize,
    input: Input,
    active_field: Rc<RefCell<usize>>,
    active: bool,
    output: FormValue<'a>,
}

#[derive(Default, Debug)]
pub struct Form<'a> {
    keymap: HashMap<KeyEvent, Action>,
    title: Cow<'a, str>,
    fields: Vec<FormField<'a>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
    cursor_pos: Position,
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    title: Cow<'a, str>,
    fields: Vec<FormField<'a>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
}
