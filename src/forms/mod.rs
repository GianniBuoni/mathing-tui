// ISSUE: dyn fields require static lifetimes
// which is causing problems in form plugins
// consider introducing a 'form lifetime

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
mod plugin;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::errors::FormErrors;
    #[allow(unused_imports)]
    pub use super::{Form, FormTui, InputField};
}

impl Form {
    const ONE_FIELD_H: u16 = 9;
    const TWO_FIELD_H: u16 = 12;
    const THREE_FIELD_H: u16 = 15;
}

#[derive(Debug)]
pub enum FormTui {
    UserFrom(Form),
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
    T: Debug + FromStr + Default + Clone,
    <T as FromStr>::Err: Debug,
{
    input: Input,
    title: Rc<str>,
    active_field: Rc<RefCell<usize>>,
    value: ParamOption<T>,
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
    request_type: RequestType,
}

#[derive(Debug, Default)]
pub struct FormBuilder {
    fields: Vec<Box<dyn Field>>,
    title: Rc<str>,
    active_field: Rc<RefCell<usize>>,
    rect: Rect,
    request_type: RequestType,
}
