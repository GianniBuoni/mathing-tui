use super::*;

impl TableData {
    fn max_rows(&self) -> usize {
        self.items.len() - 1
    }
    pub(super) fn max_pages(&self) -> i64 {
        if self.count % self.limit == 0 {
            1.max(self.count / self.limit)
        } else {
            self.count / self.limit + 1
        }
    }
    /// Increments the table page forward or backward
    pub(super) fn page_increment(&mut self, add: i64) {
        let next_page = self.pages + add;
        match next_page {
            int if int > self.max_pages() => self.pages = 1,
            int if int < 1 => self.pages = self.max_pages(),
            _ => self.pages = next_page,
        }
    }
    pub(super) fn row_increment(&mut self, add: i64) {
        let next_index = self.table_index as i64 + add;
        match next_index {
            int if int > self.max_rows() as i64 => self.table_index = 0,
            int if int < 0 => self.table_index = self.max_rows(),
            _ => self.table_index = next_index as usize,
        }
    }
    pub(super) fn add_items(&mut self, payload: DbPayload) {
        let filter = matches!(
            payload,
            DbPayload::Receipts(_) | DbPayload::Items(_) | DbPayload::Users(_)
        );
        let items: Vec<DbTable> = payload.into();
        match filter {
            true => {
                self.table_index = 0;
                self.items = items
            }
            false => {
                if self.pages == self.max_pages() {
                    self.items.push(items.first().unwrap().clone())
                }
            }
        }
    }
    pub(super) fn set_count(&mut self, payload: DbPayload) {
        if let DbPayload::Count(_, i) = payload {
            self.count = i
        }
    }
}
