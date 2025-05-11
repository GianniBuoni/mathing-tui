use std::{borrow::Cow, fmt::Debug, ops::Deref, rc::Rc};

use crate::prelude::*;

mod interactions;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{TableData, TableDisplay};
}

pub trait TableDisplay: Debug {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

#[derive(Debug, Default)]
pub struct TableData<'a, T>
where
    T: TableDisplay + Sized,
{
    title: Cow<'a, str>,
    headings: Rc<[Cow<'a, str>]>,
    items: Rc<[T]>,
    table_index: usize,
    app_index: u8,
    active: bool,
}

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn set_title(mut self, title: &'a str) -> Self {
        self.title = Cow::Borrowed(title);
        self
    }
    pub fn headings<U>(mut self, headings: U) -> Self
    where
        U: Into<Rc<[Cow<'a, str>]>>,
    {
        self.headings = headings.into();
        self
    }
    pub fn items<U>(mut self, items: U) -> Self
    where
        U: Into<Rc<[T]>>,
    {
        self.items = items.into();
        self
    }
    pub fn set_index(mut self, index: u8) -> Self {
        self.app_index = index;
        self
    }
}
