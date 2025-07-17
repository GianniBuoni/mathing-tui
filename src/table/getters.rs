use super::*;

impl TableData {
    /// Calculates the offset for a GetAll request
    pub fn get_req_offset(&self) -> i64 {
        0.max(self.pages - 1) * self.limit
    }
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn try_get_active_item(&self) -> Result<&DbTable> {
        Ok(self
            .items
            .get(self.table_index)
            .ok_or(ComponentError::NoData)?)
    }
    /// Try to formulate a refresh request based on table's
    /// current item offset and item limit
    pub fn get_refresh_reqs(&self) -> Option<DbRequest> {
        let payload = match self.table_type? {
            AppArm::Items => Some(DbPayload::ItemParams(
                ItemParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_req_offset()),
            )),
            AppArm::Receipts => Some(DbPayload::ReceiptParams(
                JoinedReceiptParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_req_offset()),
            )),
            AppArm::Users => Some(DbPayload::UserParams(UserParams::default())),
            _ => None,
        };
        payload.map(|f| {
            DbRequest::new()
                .with_req_type(RequestType::GetAll)
                .with_payload(f)
        })
    }
    pub fn goto_page(&self) -> Option<DbRequest> {
        if self.max_pages() == 1 {
            return None;
        }
        self.get_refresh_reqs()
    }
    pub fn goto_last_page(&mut self) -> Option<DbRequest> {
        self.pages = self.max_pages();
        self.get_refresh_reqs()
    }
}
