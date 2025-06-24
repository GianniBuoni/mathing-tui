use std::fmt::Display;

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
        if self.form.is_some() || self.message.is_some() {
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
                self.map_err(FormErrors::malformed("req tx"));
                return;
            };
            if let Err(err) = tx.send(req) {
                self.map_err(err);
                return;
            }
            // reset mode
            self.form = None;
            self.message = None;
            self.mode = Mode::Normal;
        }
    }

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

    pub(super) fn map_err(&mut self, err: impl Display) {
        if let Some(form) = self.form.as_mut() {
            form.map_err(err);
            return;
        }
        self.message = Some(Dialogue::message_only(err))
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
                let mut req = DbRequest::new();
                req.payload(payload).req_type(RequestType::GetAll);

                let Some(tx) = self.req_tx.clone() else {
                    self.map_err(FormErrors::malformed("req_tx"));
                    return;
                };
                if let Err(err) = tx.send(req) {
                    self.map_err(err);
                    return;
                }
            });
    }
}

impl PluginParent for HomeBuilder {}
