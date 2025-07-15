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
            let requests = submission_callback(self)?;
            requests.into_iter().try_for_each(|f| self.try_send(f))?;

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
        // unwrap the form. Should only be called if there is a form
        // to opperate on.
        let form = &mut self
            .form
            .as_mut()
            .ok_or(ComponentError::not_found("form"))?;
        form.submit()?;

        // initialize all the return data
        let mut requests = vec![DbRequest::new()];
        let req_type = form.get_req_type();
        let payload = form.try_get_payload()?;

        // if there is un update, check if we need to do
        // some calculations or push a refresh request.
        match (&payload, req_type) {
            (_, RequestType::Post) => {
                requests.append(&mut DbRequest::counts());
            }
            (DbPayload::ReceiptParams(_), RequestType::Update) => {
                self.try_subtract_store_total()?
            }
            (
                DbPayload::ItemParams(_) | DbPayload::UserParams(_),
                RequestType::Update,
            ) => {
                requests.append(&mut DbRequest::refresh());
            }
            _ => {}
        }
        // update original request to consume the payload and
        // request type
        requests
            .first_mut()
            .unwrap()
            .with_req_type(req_type)
            .with_payload(payload);

        Ok(requests)
    }

    fn try_dialogue_submit(&mut self) -> Result<Vec<DbRequest>> {
        let dialogue = self
            .message
            .as_mut()
            .ok_or(ComponentError::not_found("message"))?;

        let mut requests = vec![DbRequest::new()];
        let req_type = dialogue.get_req_type();
        let payload = dialogue.try_get_payload()?;

        match (&payload, req_type) {
            (DbPayload::ReceiptParams(_), RequestType::Delete) => {
                self.try_subtract_store_total()?
            }
            (DbPayload::ItemParams(_), RequestType::Delete) => {
                requests.append(&mut DbRequest::refresh());
            }
            (_, RequestType::Delete) => {
                requests.append(&mut DbRequest::counts());
            }
            (DbPayload::StoreTotal, _ /*Refresh to app init*/) => {
                requests.append(&mut DbRequest::init());
            }
            _ => {}
        }

        requests
            .first_mut()
            .unwrap()
            .with_req_type(req_type)
            .with_payload(payload);

        Ok(requests)
    }
}
