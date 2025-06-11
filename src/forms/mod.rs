#![allow(dead_code)]
use std::{cell::RefCell, fmt::Debug, rc::Rc, str::FromStr};

use tui_input::Input;

use crate::prelude::*;

mod builder;
mod form_data;
mod form_tui;
mod input_field;
#[cfg(test)]
mod tests;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{Form, FormTui, InputField};
}

#[derive(Debug)]
pub enum FormTui {
    ItemForm(Form),
    ReceiptForm(Form),
}

pub trait Field: Component {
    fn check_active(&mut self);
    fn assign_index(&mut self, index: usize);
    fn validate(&self) -> Result<()>;
    fn submit(&self) -> Result<()>;
}

#[derive(Default, Debug)]
pub struct InputField<T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    title: Rc<str>,
    index: usize,
    input: Input,
    active_field: Rc<RefCell<usize>>,
    active: bool,
    value: Option<Rc<RefCell<T>>>,
}

#[derive(Default, Debug)]
pub struct Form {
    title: Rc<str>,
    fields: Vec<Box<dyn Field>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
    cursor_pos: Position,
    error: Option<String>,
}

#[derive(Debug, Default)]
pub struct FormBuilder {
    title: Rc<str>,
    fields: Vec<Box<dyn Field>>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
}
