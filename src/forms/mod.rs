#![allow(dead_code)]
use std::{borrow::Cow, cell::RefCell, collections::HashMap, rc::Rc};

use tui_input::Input;

use crate::prelude::*;

mod builder;
mod component;
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

#[derive(Default, Debug)]
pub struct FormField<'a> {
    title: Cow<'a, str>,
    input: Input,
    active_field: Rc<RefCell<usize>>,
    active: bool,
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
    rect: Rect,
}
