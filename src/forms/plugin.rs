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
    pub fn edit_item(item: &StoreItem) -> Result<Self> {
        let mut form = Self::builder();
        form.with_title("Edit Item")
            .with_request_type(RequestType::Update)
            .with_form_type(AppArm::Items)
            .add_plugins(edit_item_inputs(item))?;

        form.build()
    }
    pub fn search_item() -> Result<Self> {
        let mut form = Self::builder();
        form.with_request_type(RequestType::GetAll)
            .with_form_type(AppArm::Items)
            .add_plugins(search_item_imputs)?;

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
    pub fn edit_user(user: &StoreUser) -> Result<Self> {
        let mut form = Self::builder();
        form.with_title("Edit User")
            .with_request_type(RequestType::Update)
            .with_form_type(AppArm::Users)
            .add_plugins(edit_user_inputs(user))?;

        form.build()
    }
    pub fn new_receipt(
        item: &StoreItem,
        users: Rc<[StoreUser]>,
    ) -> Result<Self> {
        let mut form = Self::builder();
        let title = format!("Add {} to Receipt", item.name);

        form.with_title(title)
            .with_request_type(RequestType::Post)
            .with_form_type(AppArm::Receipts)
            .add_plugins(new_receipt_inputs_middleware(item, users))?;

        form.build()
    }
    pub fn edit_receipt(
        receipt: &StoreJoinRow,
        users: Rc<[StoreUser]>,
    ) -> Result<Self> {
        let mut form = Self::builder();
        form.with_title("Edit Receipt")
            .with_request_type(RequestType::Update)
            .with_form_type(AppArm::Receipts)
            .add_plugins(edit_receipt_intputs(receipt, users))?;

        form.build()
    }
}
