use super::*;

impl TryFrom<DbRequest> for TableReq {
    type Error = Error;

    fn try_from(value: DbRequest) -> std::result::Result<Self, Self::Error> {
        let reqs = Vec::with_capacity(7);
        let mut new = Self {
            search_term: value.payload.get_search_term(),
            req_type: value.req_type,
            app_arm: TryInto::<AppArm>::try_into(&value.payload)?,
            reqs,
        };
        new.reqs.push(value);
        Ok(new)
    }
}

impl TableReq {
    pub fn push(&mut self, req: DbRequest) {
        self.reqs.push(req);
    }
    pub fn check_is_post(&mut self) {
        if self.req_type == RequestType::Post {
            // should swap original req (0) and get_req (1)
            self.reqs.swap(0, 1);
        }
    }
    pub(super) fn check_count(&mut self, table: &TableData) -> Option<()> {
        let req = table.count()?;
        match (self.req_type, self.app_arm) {
            (
                RequestType::Post | RequestType::Delete | RequestType::GetAll,
                _,
            ) => Some(self.push(req)),
            (RequestType::Update, AppArm::Totals) => Some(self.push(req)),
            _ => None,
        }
    }
    pub(super) fn check_retotal(&mut self) {
        if let RequestType::Reset | RequestType::Delete | RequestType::Update =
            self.req_type
        {
            self.push(DbRequest::STORE_TOTAL);
        }
    }
    pub(super) fn check_refetch(&mut self, table: &TableData) -> Option<()> {
        let req = table.get_req()?;
        let table_type = table.table_type?;

        match (self.req_type, self.app_arm, table_type) {
            // paginig or posting
            (RequestType::GetAll | RequestType::Post, _, _) => {
                Some(self.push(req))
            }
            // updating related tables
            (
                RequestType::Update | RequestType::Delete,
                AppArm::Items | AppArm::Users,
                AppArm::Receipts,
            ) => Some(self.push(req)),
            // refresh
            (RequestType::Update, AppArm::Totals, _) => Some(self.push(req)),
            _ => None,
        }
    }
}
