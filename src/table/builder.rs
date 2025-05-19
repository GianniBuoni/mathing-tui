use super::*;

#[derive(Default)]
pub struct TableBuilder<'a, T>
where
    T: TableDisplay,
{
    title: Cow<'a, str>,
    headings: Vec<Cow<'a, str>>,
    items: Vec<T>,
    app_index: usize,
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
    pub fn add_index(mut self, index: usize) -> Self {
        self.app_index = index;
        self
    }
    pub fn add_tracker(mut self, tracker: Rc<RefCell<usize>>) -> Self {
        self.tracker = tracker;
        self
    }
}

impl<'a, T> ComponentBuilder<TableBuilder<'a, T>, TableData<'a, T>>
    for TableBuilder<'a, T>
where
    T: TableDisplay,
{
    fn build(self) -> TableData<'a, T> {
        TableData::<'a, T> {
            title: self.title,
            headings: self.headings.into(),
            items: self.items.into(),
            app_index: self.app_index,
            tracker: self.tracker,
            ..Default::default()
        }
    }
    fn add_component(self, _: Box<dyn Component>) -> TableBuilder<'a, T> {
        self
    }
}
