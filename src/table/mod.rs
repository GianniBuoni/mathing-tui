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
    pub fn new(
        title: &'a str,
        headings: Rc<[Cow<'a, str>]>,
        items: impl Into<Rc<[T]>>,
        app_index: u8,
    ) -> Self {
        let items: Rc<[T]> = items.into();
        let title = Cow::Borrowed(title);

        Self {
            title,
            headings,
            items,
            table_index: 0,
            active: false,
            app_index,
        }
    }
}
