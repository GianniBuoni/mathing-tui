use std::{fmt::Display, marker::PhantomData};

use super::*;

#[derive(Default)]
pub struct TableBuilder<T>
where
    T: TableDisplay,
{
    title: Rc<str>,
    headings: Vec<Rc<str>>,
    tracker: Rc<RefCell<usize>>,
    phantom: PhantomData<T>,
}

impl<T> TableBuilder<T>
where
    T: TableDisplay,
{
    pub fn add_title(mut self, title: impl Into<Rc<str>>) -> Self {
        self.title = title.into();
        self
    }
    pub fn add_heading(mut self, heading: impl Display) -> Self {
        let heading = format!(" {heading} ");
        self.headings.push(heading.into());
        self
    }
}

impl ComponentBuilder<TableData<StoreItem>> for TableBuilder<StoreItem> {
    fn build(self) -> TableData<StoreItem> {
        TableData {
            title: self.title,
            headings: self.headings.into(),
            tracker: self.tracker,
            ..Default::default()
        }
    }
}

impl ComponentBuilder<TableData<StoreJoinRow>> for TableBuilder<StoreJoinRow> {
    fn build(self) -> TableData<StoreJoinRow> {
        TableData {
            title: self.title,
            headings: self.headings.into(),
            tracker: self.tracker,
            ..Default::default()
        }
    }
}
