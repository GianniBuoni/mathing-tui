#![allow(dead_code)]
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

use tui_input::Input;

use crate::prelude::*;

mod builder;
mod errors;
mod form_data;
mod form_tui;
mod input_field;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::errors::FormErrors;
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
    fn submit(&self) -> Result<()>;
}

#[derive(Default, Debug)]
pub struct InputField<T>
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    input: Input,
    title: Rc<str>,
    active_field: Rc<RefCell<usize>>,
    value: Option<Rc<RefCell<T>>>,
    index: usize,
    active: bool,
}

#[derive(Default, Debug)]
pub struct Form {
    error: Option<String>,
    fields: Vec<Box<dyn Field>>,
    title: Rc<str>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
    cursor_pos: Position,
}

#[derive(Debug, Default)]
pub struct FormBuilder {
    fields: Vec<Box<dyn Field>>,
    title: Rc<str>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
}
