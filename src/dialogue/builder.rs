use std::fmt::Display;

use super::*;

impl DialogueBuilder {
    pub fn with_message(&mut self, message: impl Display) -> &mut Self {
        self.message = message.to_string().into();
        self
    }
    pub fn with_req_type(&mut self, req_type: RequestType) -> &mut Self {
        self.request_type = req_type;
        self
    }
    pub fn with_from_type(&mut self, app_arm: AppArm) -> &mut Self {
        match app_arm {
            AppArm::Items => {
                self.payload =
                    Some(DbPayloadBuilder::ItemParams(ItemParams::builder()))
            }
            AppArm::Users => {
                self.payload =
                    Some(DbPayloadBuilder::UserParams(UserParams::builder()))
            }
            AppArm::Receipts => {
                self.payload = Some(DbPayloadBuilder::ReceiptParams(
                    JoinedReceiptParams::builder(),
                ))
            }
            AppArm::Totals => self.payload = Some(DbPayloadBuilder::StoreTotal),
        }
        self.form_type = Some(app_arm);
        self
    }
}

impl ComponentBuilder for DialogueBuilder {
    type Output = Dialogue;

    fn build(self) -> Result<Self::Output> {
        Ok(Self::Output {
            payload: self.payload,
            message: self.message,
            rect: Rect::new(0, 0, Dialogue::WIDTH, Dialogue::HEIGHT),
            request_type: self.request_type,
            error: false,
        })
    }
}

impl PluginParent for DialogueBuilder {}
