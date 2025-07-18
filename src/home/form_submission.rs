use super::*;

impl Home {
    pub(super) fn handle_submit(&mut self) {
        match (|| {
            let mut reqs = match true {
                _ if self.form.is_some() => self.try_form_submit()?,
                _ if self.message.is_some() && !self.is_error() => {
                    self.try_dialogue_submit()?
                }
                _ => {
                    return Aok(());
                }
            };
            self.try_add_extra_reqs(&mut reqs)?;
            reqs.into_iter().try_for_each(|f| self.try_send(f))?;

            Aok(())
        })() {
            Ok(_) => self.reset_mode(),
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn try_send(&self, req: DbRequest) -> Result<()> {
        let tx = self
            .req_tx
            .clone()
            .ok_or(ComponentError::not_found("Request tx"))?;
        tx.send(req)?;

        Ok(())
    }
    // helper methods
    fn try_form_submit(&mut self) -> Result<Vec<DbRequest>> {
        let form = &mut self
            .form
            .as_mut()
            .ok_or(ComponentError::not_found("form"))?;
        form.submit()?;

        Ok(vec![
            DbRequest::new()
                .with_req_type(form.get_req_type())
                .with_payload(form.try_get_payload()?),
        ])
    }
    fn try_dialogue_submit(&mut self) -> Result<Vec<DbRequest>> {
        let dialogue = self
            .message
            .as_mut()
            .ok_or(ComponentError::not_found("Message"))?;

        Ok(vec![
            DbRequest::new()
                .with_req_type(dialogue.get_req_type())
                .with_payload(dialogue.try_get_payload()?),
        ])
    }
    fn try_collect_refresh_reqs(&self) -> Result<Vec<DbRequest>> {
        let mut refectch_reqs = Vec::with_capacity(4);
        refectch_reqs.push(DbRequest::refresh());

        self.components
            .iter()
            .try_fold(refectch_reqs, |mut acc, f| {
                if let Some(req) = f.get_refresh_reqs() {
                    acc.push(req);
                };
                Aok(acc)
            })
    }
    fn try_add_extra_reqs(&mut self, reqs: &mut Vec<DbRequest>) -> Result<()> {
        let first_req = reqs.first().unwrap();
        let (payload, req_type) = (&first_req.payload, first_req.req_type);
        let app_arm: AppArm = payload.try_into()?;

        match (app_arm, req_type) {
            // Get conditions
            (AppArm::Items, RequestType::GetAll) => {
                let DbPayload::ItemParams(payload) = payload else {
                    return Ok(());
                };
                if let Some(search_term) = payload.search_filter.as_ref() {
                    let table = self
                        .get_mut_table_from_type(AppArm::Items)
                        .ok_or(ComponentError::not_found("Item table"))?;

                    table.set_search(search_term.to_owned());
                    table.reset_pages();
                    reqs.append(&mut DbRequest::counts(Some(
                        search_term.to_owned(),
                    )));
                }
            }
            // Post conditions
            (_, RequestType::Post) => {
                'paging_check: {
                    let Some(table) = self.get_mut_table_from_type(app_arm)
                    else {
                        break 'paging_check;
                    };
                    if let Some(req) = table.goto_last_page() {
                        reqs.push(req);
                        reqs.reverse();
                    }
                }
                let search = self
                    .get_mut_table_from_type(AppArm::Items)
                    .ok_or(Error::msg(""))?
                    .last_search
                    .as_ref()
                    .map(|f| f.to_string());

                reqs.append(&mut DbRequest::counts(search))
            }
            // Update conditions
            (AppArm::Receipts, RequestType::Update) => {
                self.try_subtract_store_total()?
            }
            (AppArm::Items | AppArm::Users, RequestType::Update) => {
                reqs.append(&mut self.try_collect_refresh_reqs()?)
            }
            // Deletion Conditions
            (_, RequestType::Delete) => {
                match app_arm {
                    AppArm::Items | AppArm::Users => {
                        reqs.append(&mut self.try_collect_refresh_reqs()?)
                    }
                    AppArm::Receipts => self.try_subtract_store_total()?,
                    _ => {}
                }
                let search = self
                    .get_mut_table_from_type(AppArm::Items)
                    .ok_or(Error::msg(""))?
                    .last_search
                    .as_ref()
                    .map(|f| f.to_string());

                reqs.append(&mut DbRequest::counts(search))
            }
            // Reset conditons
            (AppArm::Totals, _) => {
                'search_check: {
                    let Some(items) =
                        self.get_mut_table_from_type(AppArm::Items)
                    else {
                        break 'search_check;
                    };
                    items.last_search = None;
                }
                reqs.append(&mut DbRequest::init())
            }
            _ => return Ok(()),
        }
        Ok(())
    }
}
