use super::*;

impl Form {
    pub fn new_item() -> Option<FormTui> {
        let mut form = Self::new();

        form.add_title("New Item")
            .add_rect(Self::form_rect(Self::TWO_FIELD_H))
            .add_request_type(RequestType::Post);

        let mut form = FormTui::ItemForm(form);

        InputField::<String>::new("Item Name").plugin(&mut form);
        InputField::<f64>::new("Item Price").plugin(&mut form);

        Some(form.map(|i| i.init()))
    }
    pub fn update_item(item: &StoreItem) -> FormTui {
        let _ = item;
        let mut form = Self::new();

        form.add_title("Update Item")
            .add_rect(Self::form_rect(Self::TWO_FIELD_H))
            .add_request_type(RequestType::Update);

        let mut form = FormTui::ItemForm(form);

        // TODO:: make new field for id's. SHOULD NOT BE EDITABLE.
        // TOOD:: make with value method for input values
        InputField::<String>::new("Item Name").plugin(&mut form);
        InputField::<f64>::new("Item Price").plugin(&mut form);

        todo!()
    }
    pub fn delete_item(item: &StoreItem) -> FormTui {
        let _ = item;
        todo!()
    }
}
