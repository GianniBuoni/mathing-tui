use super::*;

mod new_receipt;

impl Form {
    pub fn new_item() -> Option<Self> {
        let mut form = Self::builder();
        form.with_title("New Item")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Items)
            .add_plugins(new_item_inputs);

        Some(form.build())
    }
    pub fn new_user() -> Option<Self> {
        let mut form = Self::builder();
        form.with_title("New User")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Users)
            .add_plugins(new_user_inputs);

        Some(form.build())
    }
}
