use super::*;

impl Home {
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
                }
            });
    }
}
