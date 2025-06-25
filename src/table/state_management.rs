use super::*;

impl TableData {
    fn max(&self) -> usize {
        self.items.len() - 1
    }
    pub(super) fn add_items(&mut self, items: Vec<DbTable>) {
        match items.len() {
            1 => {
                self.items.push(items.first().unwrap().clone());
            }
            _ => self.items = items,
        }
    }
    pub(super) fn next_row(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }
    pub(super) fn prev_row(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }
}
