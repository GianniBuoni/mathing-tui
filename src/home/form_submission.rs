use super::*;

impl Home {
    pub(super) fn handle_submit(&mut self) {
        match (|| {
            let req = match true {
                _ if self.form.is_some() => self.try_form_submit()?,
                _ if self.message.is_some() && !self.is_error() => {
                    self.try_dialogue_submit()?
                }
                _ => {
                    return Aok(());
                }
            };
            let reqs = self.try_add_extra_reqs(req)?;
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
    fn try_form_submit(&mut self) -> Result<DbRequest> {
        let form = &mut self
            .form
            .as_mut()
            .ok_or(ComponentError::not_found("form"))?;
        form.submit()?;

        Ok(DbRequest::new()
            .with_req_type(form.get_req_type())
            .with_payload(form.try_get_payload()?))
    }
    fn try_dialogue_submit(&mut self) -> Result<DbRequest> {
        let dialogue = self
            .message
            .as_mut()
            .ok_or(ComponentError::not_found("Message"))?;

        Ok(DbRequest::new()
            .with_req_type(dialogue.get_req_type())
            .with_payload(dialogue.try_get_payload()?))
    }
    fn try_add_extra_reqs(&mut self, req: DbRequest) -> Result<Vec<DbRequest>> {
        let (app_arm, req_type, search_term) = req.try_descruct()?;

        let mut reqs = Vec::with_capacity(7);
        reqs.push(req);

        self.components.iter_mut().for_each(|f| {
            if let Some(mut extra_reqs) =
                f.collect_reqs((app_arm, req_type, search_term.clone()))
            {
                reqs.append(&mut extra_reqs);
            }
        });
        if req_type == RequestType::Post {
            // TODO: write a test for this?
            // should swap original req (0) and cascading get_req
            // from collect req (1)
            reqs.swap(0, 1);
        }
        if matches!(
            (app_arm, req_type),
            (AppArm::Receipts, RequestType::Update | RequestType::Delete)
        ) {
            self.try_subtract_store_total()?;
        }

        Ok(reqs)
    }
}
