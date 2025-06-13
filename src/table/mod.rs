use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::prelude::*;

mod builder;
mod data;
mod interactions;
mod plugin;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub(crate) use super::plugin::plugin as table_plugin;
    #[allow(unused_imports)]
    pub use super::{TableData, TableDisplay};
}

pub trait TableDisplay: Debug + Default {
    fn ref_array(&self) -> Vec<Cell>;
}

// TODO figure out a way to add rows to items after build time
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
