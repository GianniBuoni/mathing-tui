use super::*;

impl Home {
    pub(super) fn handle_submit(&mut self) {
        match (|| {
            let submission_callback = match true {
                _ if self.form.is_some() => Self::try_form_submit,
                _ if self.message.is_some() && !self.is_error() => {
                    Self::try_dialogue_submit
                }
                _ => {
                    return Aok(());
                }
            };
            let mut reqs = submission_callback(self)?;
            let first_req = reqs.first().unwrap();
            let (app_arm, req_type) = (
                TryInto::<AppArm>::try_into(&first_req.payload)?,
                first_req.req_type,
            );
            self.try_add_conditional_reqs(&mut reqs, app_arm, req_type)?;
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
    fn try_add_conditional_reqs(
        &mut self,
        reqs: &mut Vec<DbRequest>,
        app_arm: AppArm,
        req_type: RequestType,
    ) -> Result<()> {
        match (app_arm, req_type) {
            // Post conditions
            (_, RequestType::Post) => {
                (|| {
                    let Some(table) = self.get_mut_table_from_type(app_arm)
                    else {
                        return;
                    };
                    if let Some(req) = table.goto_last_page() {
                        reqs.push(req);
                        reqs.reverse();
                    }
                })();
                reqs.append(&mut DbRequest::counts())
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
                reqs.append(&mut DbRequest::counts())
            }
            (AppArm::Totals, _) => reqs.append(&mut DbRequest::init()),
            _ => return Ok(()),
        }
        Ok(())
    }
}
