use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::prelude::*;

mod builder;
mod data;
mod interactions;
mod render;
mod table_tui;
#[cfg(test)]
mod tests;

pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{TableData, TableDisplay, TableTui};
}

pub trait TableDisplay: Debug + Default {
    fn ref_array(&self) -> Vec<Cell>;
}

#[derive(Debug)]
pub enum TableTui {
    Items(TableData<StoreItem>),
    Receipt(TableData<StoreJoinRow>),
}

#[derive(Debug, Default)]
pub struct TableData<T>
where
    T: TableDisplay,
{
    title: Rc<str>,
    items: Vec<T>,
    headings: Rc<[Rc<str>]>,
    table_index: usize,
    app_index: usize,
    tracker: Rc<RefCell<usize>>,
    active: bool,
}
