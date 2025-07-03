use super::*;

impl TableData {
    fn max(&self) -> usize {
        self.items.len() - 1
    }
    pub(super) fn max_pages(&self) -> i64 {
        if self.count % self.limit == 0 {
            1.max(self.count / self.limit)
        } else {
            self.count / self.limit + 1
        }
    }
    /// Calculates the offset for a GetAll request
    pub(super) fn get_req_offset(&self) -> i64 {
        0.max(self.pages - 1) * self.limit
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
                if self.pages == self.max_pages() {
                    self.items.push(items.first().unwrap().clone());
                }
            }
        }
    }
    pub(super) fn set_count(&mut self, payload: DbPayload) {
        if let DbPayload::Count(_, i) = payload {
            self.count = i
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
    pub(super) fn next_page(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.pages < self.max_pages() {
            self.pages += 1
        } else {
            self.pages = 1
        }
        // make req?
    }
    pub(super) fn prev_page(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.pages > 1 {
            self.pages -= 1
        } else {
            self.pages = self.max_pages()
        }
        // make req?
    }
}
