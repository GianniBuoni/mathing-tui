use std::{borrow::Cow, ops::Deref, rc::Rc};

use crate::prelude::*;

mod interactions;
mod mock_structs;
mod render;

pub(crate) mod prelude {
    pub(crate) use super::TableData;
    pub(crate) use super::mock_structs::{MockItems, MockReceipt};
}

pub trait TableDisplay {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

pub struct TableData<'a, T>
where
    T: TableDisplay + Sized,
{
    headings: Rc<[Cow<'a, str>]>,
    items: Rc<[T]>,
    table_index: usize,
    active: bool,
}

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn new(
        headings: Rc<[Cow<'a, str>]>,
        items: impl Into<Rc<[T]>>,
        active: bool,
    ) -> Self {
        let items: Rc<[T]> = items.into();

        Self {
            headings,
            items,
            table_index: 0,
            active,
        }
    }
}
