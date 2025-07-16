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
    /// Produce a GetAll Request given a table's limit and table_type
    pub fn get_paging_req(&self) -> Option<DbRequest> {
        // early return conditions
        let table_type = self.table_type?;
        if self.max_pages() == 1 {
            return None;
        }

        let payload = match table_type {
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
            _ => None,
        };
        payload.map(|f| {
            let mut req = DbRequest::new();
            req.with_req_type(RequestType::GetAll).with_payload(f);
            req
        })
    }
    // Try to formulate a refresh request based on table's
    // current item offset and item limit
    pub fn try_get_refresh(&self) -> Result<DbRequest> {
        let Some(table_type) = self.table_type else {
            return Err(ComponentError::not_found("Table type").into());
        };

        let payload = match table_type {
            AppArm::Items => DbPayload::ItemParams(
                ItemParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_req_offset()),
            ),
            AppArm::Receipts => DbPayload::ReceiptParams(
                JoinedReceiptParams::default()
                    .with_limit(self.limit)
                    .with_offset(self.get_req_offset()),
            ),
            AppArm::Users => DbPayload::UserParams(UserParams::default()),
            _ => {
                return Err(Error::msg(
                    "Tables only support AppArms: Items, Receipts, and Users",
                ));
            }
        };
        let mut req = DbRequest::new();
        req.with_req_type(RequestType::GetAll).with_payload(payload);

        Ok(req)
    }
}
