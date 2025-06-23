use super::*;

impl Form {
    pub fn new_item() -> Result<Self> {
        let mut form = Self::builder();
        form.with_title("New Item")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Items)
            .add_plugins(new_item_inputs)?;

        form.build()
    }
    pub fn new_user() -> Result<Self> {
        let mut form = Self::builder();
        form.with_title("New User")
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Users)
            .add_plugins(new_user_inputs)?;

        form.build()
    }
    pub fn new_receipt(
        item: &StoreItem,
        users: Vec<&StoreUser>,
    ) -> Result<Self> {
        let mut form = Self::builder();

        let title = format!("Add {} to Receipt", item.name);

        form.with_title(title)
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Receipts)
            .add_plugins(new_receipt_inputs_middleware(item, users))?;

        form.build()
    }
    pub fn delete_item(item: &StoreItem) -> Result<Self> {
        let mut form = Self::builder();

        let title = format!("Delete {}", item.name);

        form.with_title(title)
            .with_request_type(RequestType::Delete)
            .with_form_type(AppArm::Items);

        form.build()
    }
}
