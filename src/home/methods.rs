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
        let mut current = self.component_tracker.borrow_mut();

        match *current as i32 + add {
            int if int > max as i32 => *current = 0,
            int if int < 0 => *current = max,
            _ => *current = (*current as i32 + add) as usize,
        }
    }
    pub(super) fn handle_submit(&mut self) {
        if let Some(form) = self.form.as_mut() {
            if let Err(e) = form.submit() {
                form.map_err(Some(e));
                return;
            }

            if let Some(params) = self.from_params.as_ref() {
                let payload = params.build();
                let req = DbRequest::new()
                    .req_type(form.get_req_type())
                    .payload(payload);

                if let Some(tx) = self.req_tx.clone() {
                    if let Err(err) = tx.send(req) {
                        let err = anyhow::Error::msg(err.to_string());
                        form.map_err(Some(err));
                        return;
                    }
                }
            };

            self.form = None;
            self.from_params = None;
            self.mode = Mode::Normal;
        }
    }

    /// [`Home`]'s init method is responsible for making all the initial
    /// requests to the Db
    pub fn init(&mut self) {
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
