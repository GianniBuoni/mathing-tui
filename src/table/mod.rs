use std::{borrow::Cow, cell::RefCell, fmt::Debug, ops::Deref, rc::Rc};

use crate::prelude::*;
use builder::TableBuilder;

mod builder;
mod interactions;
mod render;
#[cfg(test)]
mod tests;

pub mod prelude {
    pub use super::{TableData, TableDisplay};
}

pub trait TableDisplay: Debug + Default {
    fn ref_array(&self) -> Vec<Cow<str>>;
}

#[derive(Debug, Default)]
pub struct TableData<'a, T>
where
    T: TableDisplay,
{
    title: Cow<'a, str>,
    headings: Rc<[Cow<'a, str>]>,
    items: Rc<[T]>,
    table_index: usize,
    app_index: usize,
    tracker: Rc<RefCell<usize>>,
    active: bool,
}

impl<'a, T> TableData<'a, T>
where
    T: TableDisplay,
{
    pub fn new_builder() -> TableBuilder<'a, T> {
        TableBuilder::default()
    }
}

impl<T> Component for TableData<'_, T>
where
    T: TableDisplay,
{
    fn handle_key_events(&mut self, _key: KeyEvent) -> Option<Action> {
        None
    }
    fn update(&mut self, _action: Option<Action>) {
        let current_index = self.tracker.borrow();
        if current_index.deref() == &self.app_index {
            self.active = true;
        }
    }
    fn draw(&mut self, _frame: &mut Frame, _rect: Rect) {
        ()
    }
}
