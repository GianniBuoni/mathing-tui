use super::*;

#[derive(Default)]
pub struct TableBuilder<'a, T>
where
    T: TableDisplay,
{
    title: Cow<'a, str>,
    headings: Vec<Cow<'a, str>>,
    items: Vec<T>,
    tracker: Rc<RefCell<usize>>,
}

impl<'a, T> TableBuilder<'a, T>
where
    T: TableDisplay,
{
    pub fn add_title(mut self, title: &'a str) -> Self {
        self.title = Cow::Borrowed(title);
        self
    }
    pub fn add_heading(mut self, heading: &'a str) -> Self {
        self.headings.push(Cow::Owned(format!(" {heading} ")));
        self
    }
    pub fn add_item(mut self, item: T) -> Self {
        self.items.push(item);
        self
    }
}

impl<'a, T> ComponentBuilder<TableData<'a, T>> for TableBuilder<'a, T>
where
    T: TableDisplay,
{
    fn build(self) -> TableData<'a, T> {
        TableData::<'a, T> {
            title: self.title,
            headings: self.headings.into(),
            items: self.items,
            tracker: self.tracker,
            ..Default::default()
        }
    }
}
