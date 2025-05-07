use super::*;

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    pub fn max(&self) -> usize {
        self.items.len() - 1
    }
}

impl<T> Model for TableData<'_, T>
where
    T: TableDisplay,
{
    fn title(&self) -> Cow<str> {
        Cow::Owned(format!(" [{}] {} ", self.app_index, self.title))
    }
    fn is_active(&self) -> bool {
        self.active
    }
    fn index(&self) -> u8 {
        self.app_index
    }
    fn toggle(&mut self) {
        self.active = !self.active;
    }
    fn next_row(&mut self) {
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }
    fn prev_row(&mut self) {
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }
}
