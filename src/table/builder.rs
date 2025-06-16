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
    fn build(self) -> TableData<T> {
        TableData {
            title: self.title,
            headings: self.headings.into(),
            tracker: self.tracker,
            ..Default::default()
        }
    }
}

impl ComponentBuilder for TableBuilder<StoreItem> {
    type Output = TableData<StoreItem>;

    fn build(self) -> Self::Output {
        self.build()
    }
}

impl ComponentBuilder for TableBuilder<StoreJoinRow> {
    type Output = TableData<StoreJoinRow>;

    fn build(self) -> Self::Output {
        self.build()
    }
}

impl ComponentBuilder for TableBuilder<StoreUser> {
    type Output = TableData<StoreUser>;

    fn build(self) -> Self::Output {
        self.build()
    }
}
