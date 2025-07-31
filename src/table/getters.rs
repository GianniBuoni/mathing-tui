use super::*;

impl TableData {
    /// Calculates the offset for a GetAll request
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn try_get_active_item(&self) -> Result<&DbTable> {
        Ok(self
            .items
            .get(self.table_index)
            .ok_or(ComponentError::NoData)?)
    }
    /// Handles collecting cascading requests for a table based on
    /// an initial request from a form or dialogue.
    /// Any necessary paging, and filter reseting is also handled here
    pub fn collect_reqs(&mut self, table_req: &mut TableReq) {
        if let Some(search_term) = table_req.search_term.as_ref() {
            self.last_search = Some(search_term.clone());
        }
        // check if state needs to change
        match (table_req.req_type, table_req.app_arm) {
            (RequestType::Post, _) => self.page_to_last(),
            (RequestType::GetAll, _) => {
                if self.last_search.is_some() {
                    self.page_to_first();
                }
            }
            (RequestType::Update, AppArm::Totals) => {
                self.page_to_first();
                self.last_search = None;
            }
            _ => {}
        }
        // match which reqs need to be made
        table_req.check_refetch(self);
        table_req.check_count(self);
        table_req.check_retotal();
    }
    /// Return a GetAll request for the current table
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
    /// Return a Count request for current table
    pub(super) fn count(&self) -> Option<DbRequest> {
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
    /// Return a paging request for current tables
    pub fn handle_paging(
        &mut self,
        action: Option<Action>,
    ) -> Option<DbRequest> {
        match self.is_active() {
            true => {
                self.handle_action(action);
                self.get_req()
            }
            false => None,
        }
    }
}
