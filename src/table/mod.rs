use std::{borrow::Cow, fmt::Debug, ops::Deref, rc::Rc};

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
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        None
    }
    fn update(&mut self, action: Option<Action>) {
        ()
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        ()
    }
}
