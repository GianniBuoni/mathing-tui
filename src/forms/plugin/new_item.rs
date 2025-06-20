use super::*;

impl Form {
    pub fn new_item() -> Option<Self> {
        let mut form = Self::builder();
        form.with_title("New Item")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Items)
            .add_plugins(new_item);

        Some(form.build())
    }
}
