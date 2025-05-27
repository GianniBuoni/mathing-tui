#![allow(dead_code)]
use std::{borrow::Cow, cell::RefCell, fmt::Debug, rc::Rc, str::FromStr};

use tui_input::Input;

use crate::prelude::*;

mod builder;
mod form_data;
mod form_tui;
mod form_value;
mod input_field;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{Form, FormTui, InputField};
}

#[derive(Debug)]
pub enum FormTui<'a> {
    ItemForm(Form<'a>),
    ReceiptForm(Form<'a>),
}

pub trait Field: Component {
    fn check_active(&mut self);
    fn assign_index(&mut self, index: usize);
    fn validate(&self) -> Result<()>;
    fn submit(&self) -> Result<()>;
}

#[derive(Default, Debug)]
pub struct InputField<'a, T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    title: Cow<'a, str>,
    index: usize,
    input: Input,
    active_field: Rc<RefCell<usize>>,
    active: bool,
    value: Option<Rc<RefCell<T>>>,
}

#[derive(Default, Debug)]
pub struct Form<'a> {
    title: Cow<'a, str>,
    fields: Vec<Box<dyn Field>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
    cursor_pos: Position,
    error: Option<Cow<'a, str>>,
}

#[derive(Debug, Default)]
pub struct FormBuilder<'a> {
    title: Cow<'a, str>,
    fields: Vec<Box<dyn Field>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
}
