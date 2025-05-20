use std::{borrow::Cow, cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use crate::prelude::*;

mod builder;
mod data;
mod interactions;
mod render;
mod table_tui;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{TableData, TableDisplay, TableTui};
}

pub trait TableDisplay: Debug + Default {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

#[derive(Debug)]
pub enum TableTui<'a> {
    Items(TableData<'a, StoreItem>),
    Receipt(TableData<'a, StoreJoinRow>),
}

#[derive(Debug, Default)]
pub struct TableData<'a, T>
where
    T: TableDisplay,
{
    title: Cow<'a, str>,
    headings: Rc<[Cow<'a, str>]>,
    items: Vec<T>,
    table_index: usize,
    app_index: usize,
    tracker: Rc<RefCell<usize>>,
    active: bool,
}
