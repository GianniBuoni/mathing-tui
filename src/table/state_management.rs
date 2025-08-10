use super::*;

impl TableData {
    fn max_rows(&self) -> usize {
        self.items.len() - 1
    }
    /// sets the [`next page`] field
    pub(super) fn page_increment(&mut self, add_page: i64) {
        let next_page = self.current_page + add_page;
        match next_page {
            int if int > self.max_pages() => self.next_page = 1,
            int if int < 1 => self.next_page = self.max_pages(),
            _ => self.next_page = next_page,
        };
    }
    pub(super) fn max_pages(&self) -> i64 {
        if self.count % self.limit == 0 {
            1.max(self.count / self.limit)
        } else {
            self.count / self.limit + 1
        }
    }
    pub(super) fn page_to_first(&mut self) {
        if self.max_pages() == 1 {
            return;
        }
        self.next_page = 1;
    }
    pub(super) fn page_to_last(&mut self) {
        // determines if the a new page needs to be added
        // based on item count
        let next_page = match self.count {
            0 => {
                return;
            }
            count if count % self.limit == 0 => self.max_pages() + 1,
            _ => self.max_pages(),
        };
        self.next_page = next_page;
    }
    pub(super) fn get_next_offset(&self) -> i64 {
        0.max(self.next_page - 1) * self.limit
    }
    /// Increments the table page forward or backward
    pub(super) fn row_increment(&mut self, add: i64) {
        let next_index = self.table_index as i64 + add;
        match next_index {
            int if int > self.max_rows() as i64 => self.table_index = 0,
            int if int < 0 => self.table_index = self.max_rows(),
            _ => self.table_index = next_index as usize,
        }
    }
    /// Consumes the payload to be added to the table
    pub(super) fn add_items(&mut self, payload: DbPayload) {
        let filter = matches!(
            payload,
            DbPayload::Receipts(_) | DbPayload::Items(_) | DbPayload::Users(_)
        );
        let items: Vec<DbTable> = payload.into();
        match filter {
            true => {
                self.items = items;
                self.current_page = self.next_page;
                self.table_index = 0
            }
            false => {
                self.current_page = self.next_page;
                if self.current_page == self.max_pages() {
                    self.items.push(items.first().unwrap().to_owned());
                    self.table_index = self.max_rows()
                }
            }
        }
    }
    pub(super) fn set_count(&mut self, payload: &DbPayload) {
        if let DbPayload::Count(_, i) = payload {
            self.count = *i
        }
    }
    pub fn try_subtract_store_total(&self) -> Result<()> {
        if let Some(AppArm::Receipts) = self.table_type {
            let current_r = self.try_get_active_item()?.try_get_receipt()?;

            StoreTotal::try_get()?
                .lock()
                .map_err(|_| AppError::StoreTotalMutex)?
                .subtract(current_r.try_calc()?);
        }
        Ok(())
    }
}
