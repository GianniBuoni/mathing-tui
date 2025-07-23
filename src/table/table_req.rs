use super::*;

impl TableData {
    /// Formulates a GetAll request for the current table
    pub fn get_req(&self) -> Option<DbRequest> {
        let payload = match self.table_type? {
            AppArm::Items => {
                let mut base_param = ItemParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_next_offset());
                if let Some(search_term) = self.last_search.as_ref() {
                    base_param = base_param.with_search(search_term);
                }
                Some(DbPayload::ItemParams(base_param))
            }
            AppArm::Receipts => Some(DbPayload::ReceiptParams(
                JoinedReceiptParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_next_offset()),
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
    /// Formulates a Count request for current table
    fn count(&self) -> Option<DbRequest> {
        let payload = match self.table_type? {
            AppArm::Items => {
                let mut item_param = ItemParams::default();
                if let Some(search_term) = self.last_search.as_ref() {
                    item_param = item_param.with_search(search_term);
                }
                Some(DbPayload::ItemParams(item_param))
            }
            AppArm::Receipts => {
                Some(DbPayload::ReceiptParams(JoinedReceiptParams::default()))
            }
            AppArm::Users => Some(DbPayload::UserParams(UserParams::default())),
            _ => None,
        };
        payload.map(|f| {
            DbRequest::new()
                .with_req_type(RequestType::Count)
                .with_payload(f)
        })
    }
    /// Handles collecting cascading requests for a table based on
    /// an initial request from a form or dialogue.
    /// Any necessary paging, and filter reseting is also handled here
    pub fn collect_reqs(
        &mut self,
        req_seed: (AppArm, RequestType, Option<Rc<str>>),
    ) -> Option<Vec<DbRequest>> {
        let (app_arm, req_type, search_term) = req_seed;

        if let Some(search_term) = search_term.as_ref() {
            self.last_search = Some(search_term.clone());
        }
        let table_type = self.table_type?;
        // TODO: clean up matches to be a little more readable?
        match (app_arm, req_type, table_type) {
            // Get condition(s)
            (_, RequestType::GetAll, _) => {
                if search_term.as_ref().is_some() {
                    self.page_to_first();
                }
                Some(vec![self.get_req()?, self.count()?])
            }
            // Post condition(s)
            (_, RequestType::Post, _) => match table_type {
                table_type if table_type == app_arm => {
                    self.page_to_last();
                    Some(vec![self.get_req()?, self.count()?])
                }
                _ => None,
            },
            // Update condition(s)
            (AppArm::Receipts, RequestType::Update, _) => None,
            (
                AppArm::Items | AppArm::Users,
                RequestType::Update,
                AppArm::Receipts,
            ) => Some(vec![self.get_req()?, DbRequest::STORE_TOTAL]),
            (AppArm::Totals, RequestType::Update, _) => {
                self.page_to_first();
                self.last_search = None;
                Some(vec![self.get_req()?, self.count()?])
            }
            // Delete condition(s)
            (AppArm::Receipts, RequestType::Delete, _) => None,
            (
                AppArm::Items | AppArm::Users,
                RequestType::Delete,
                AppArm::Receipts,
            ) => Some(vec![
                self.get_req()?,
                self.count()?,
                DbRequest::STORE_TOTAL,
            ]),
            (_, RequestType::Delete, _) => Some(vec![self.count()?]),
            // Reset condition(s)
            (_, RequestType::Reset, _) => Some(vec![DbRequest::STORE_TOTAL]),
            _ => None,
        }
    }
}
