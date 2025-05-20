use super::*;

impl<T> TableData<'_, T> where T: TableDisplay {}

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    pub fn max(&self) -> usize {
        self.items.len() - 1
    }

    pub fn title(&self) -> Cow<str> {
        Cow::Owned(format!(" [{}] {} ", self.app_index, self.title))
    }

    pub fn next_row(&mut self) {
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }

    pub fn prev_row(&mut self) {
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }

    pub fn check_active(&mut self) {
        let current_index = self.tracker.borrow();
        self.active = current_index.deref() == &self.app_index;
    }
}
