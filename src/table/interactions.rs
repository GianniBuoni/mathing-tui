use super::*;

impl<T> TableData<'_, T>
where
    T: TableDisplay,
{
    fn max(&self) -> usize {
        self.items.len() - 1
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
    pub fn sync_block(&mut self, active: bool) {
        self.active = active;
    }
}
