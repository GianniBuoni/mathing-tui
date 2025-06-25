use super::*;

impl Home {
    pub(super) fn handle_submit(&mut self) {
        if self.form.is_some() || (self.message.is_some() && !self.is_error()) {
            // init request
            let mut req = DbRequest::new();
            // try any submission
            if let Err(err) = self.try_form_submit(&mut req) {
                self.map_err(err);
                return;
            }
            if let Err(err) = self.try_dialogue_submit(&mut req) {
                self.map_err(err);
                return;
            }
            let Some(tx) = self.req_tx.clone() else {
                self.map_err(FormError::malformed("req tx"));
                return;
            };
            if let Err(err) = tx.send(req) {
                self.map_err(err);
                return;
            }
        }
        // defer to resetting
        self.form = None;
        self.message = None;
        self.mode = Mode::Normal;
    }

    // helper methods
    fn try_form_submit(&mut self, req: &mut DbRequest) -> Result<()> {
        let Some(form) = self.form.as_mut() else {
            return Ok(());
        };
        form.submit()?;
        req.req_type(form.get_req_type())
            .payload(form.try_get_payload()?);

        Ok(())
    }

    fn try_dialogue_submit(&mut self, req: &mut DbRequest) -> Result<()> {
        let Some(dialogue) = self.message.as_mut() else {
            return Ok(());
        };
        req.req_type(dialogue.get_req_type())
            .payload(dialogue.try_get_payload()?);

        Ok(())
    }
}
