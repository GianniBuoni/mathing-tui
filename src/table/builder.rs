use super::*;

#[derive(Default)]
pub struct TableBuilder<T>
where
    T: TableDisplay,
{
    title: Rc<str>,
    headings: Vec<String>,
    items: Vec<T>,
    tracker: Rc<RefCell<usize>>,
}

impl<T> TableBuilder<T>
where
    T: TableDisplay,
{
    pub fn add_title(mut self, title: impl Into<Rc<str>>) -> Self {
        self.title = title.into();
        self
    }
    pub fn add_heading(mut self, heading: impl ToString) -> Self {
        self.headings.push(heading.to_string());
        self
    }
    pub fn add_item(mut self, item: T) -> Self {
        self.items.push(item);
        self
    }
}

impl<T> ComponentBuilder<TableData<T>> for TableBuilder<T>
where
    T: TableDisplay,
{
    fn build(self) -> TableData<T> {
        TableData::<T> {
            title: self.title,
            headings: self.headings.into(),
            items: self.items,
            tracker: self.tracker,
            ..Default::default()
        }
    }
}
