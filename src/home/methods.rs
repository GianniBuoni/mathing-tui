use super::*;

impl Home {
    pub fn builder() -> HomeBuilder {
        HomeBuilder::default()
    }
    pub(super) fn cycle_active(&mut self, add: i32) {
        if self.components.is_empty() {
            return;
        }

        let max = self.components.len() - 1;
        let new_index = self.component_tracker.inner() as i32 + add;

        match new_index {
            int if int > max as i32 => self.component_tracker.go_to(0),
            int if int < 0 => self.component_tracker.go_to(max),
            _ => self.component_tracker.go_to(new_index as usize),
        }
    }
    pub(super) fn handle_submit(&mut self) {
        // return early if there is no form
        let Some(form) = self.form.as_mut() else {
            return;
        };
        // unwrap the payload; if none map an err to the form
        let Some(payload) = form.get_payload() else {
            form.map_err(Some(FormErrors::malformed("payload").into()));
            return;
        };
        // unwrap tx; if none map err
        let Some(tx) = self.req_tx.clone() else {
            form.map_err(Some(FormErrors::malformed("req tx").into()));
            return;
        };
        // try a submit; if there is an err, map it to the form
        if let Err(e) = form.submit() {
            form.map_err(Some(e));
            return;
        }
        // no errors -> start building req
        let req = DbRequest::new()
            .req_type(form.get_req_type())
            .payload(payload);

        // send req; if err map err
        if let Err(err) = tx.send(req) {
            let err = anyhow::Error::msg(err.to_string());
            form.map_err(Some(err));
            return;
        }
        // reset mode
        self.form = None;
        self.mode = Mode::Normal;
    }

    /// [`Home`]'s init method is responsible for making all the initial
    /// requests to the Db
    pub fn fetch_all(&mut self) {
        let item_payload =
            DbPayload::ItemParams(ItemParams::builder().offset(0).build());
        let user_payload = DbPayload::UserParams(UserParams::builder().build());
        let r_payload = DbPayload::ReceiptParams(
            JoinedReceiptParams::builder().offset(0).build(),
        );

        [item_payload, user_payload, r_payload]
            .into_iter()
            .for_each(|payload| {
                let req = DbRequest::new()
                    .payload(payload)
                    .req_type(RequestType::GetAll);
                // TODO: add real error handling
                let _ = self.req_tx.as_ref().unwrap().send(req);
            });
    }
}

impl PluginParent for HomeBuilder {}
