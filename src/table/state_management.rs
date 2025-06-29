use super::*;

impl TableData {
    fn max(&self) -> usize {
        self.items.len() - 1
    }
    pub(super) fn add_items(&mut self, payload: DbPayload) {
        let filter = matches!(
            payload,
            DbPayload::Receipts(_) | DbPayload::Items(_) | DbPayload::Users(_)
        );
        let items: Vec<DbTable> = payload.into();
        match filter {
            true => self.items = items,
            false => {
                self.items.push(items.first().unwrap().clone());
            }
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
