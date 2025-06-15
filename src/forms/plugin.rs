use super::*;

impl Form {
    fn form_rect(height: u16) -> Rect {
        Rect::new(0, 0, 60, height)
    }

    pub fn new_item_form() -> (Self, DbPayloadBuilder) {
        let payload_builder =
            ItemParamsBuilder::default().item_name("").item_price(0.);
        let mut form = Self::new_builder();

        form.add_title("New Item")
            .add_rect(Self::form_rect(Self::TWO_FIELD_H))
            .add_request_type(RequestType::Post);

        if let Ok(name_value) = payload_builder.item_name.clone_inner() {
            let field =
                InputField::<String>::new("Item Name").map_value(name_value);
            form.add_field(field);
        } else {
            let mut form = form.build();
            form.error = Some(FormErrors::unmapped("Item name").to_string());
            return (form, DbPayloadBuilder::None);
        }

        if let Ok(price_value) = payload_builder.item_price.clone_inner() {
            let field =
                InputField::<f64>::new("Item Price").map_value(price_value);
            form.add_field(field);
        } else {
            let mut form = form.build();
            form.error = Some(FormErrors::unmapped("Item Price").to_string());
            return (form, DbPayloadBuilder::None);
        }

        (form.build(), DbPayloadBuilder::ItemParams(payload_builder))
    }

    pub fn new_user_form() -> (Self, DbPayloadBuilder) {
        let payload_builder = UserParamsBuilder::default().user_name("");
        let mut form = Self::new_builder();

        form.add_title("New User")
            .add_rect(Self::form_rect(Self::ONE_FIELD_H))
            .add_request_type(RequestType::Post);

        if let Ok(name_value) = payload_builder.name.clone_inner() {
            let field =
                InputField::<String>::new("User Name").map_value(name_value);
            form.add_field(field);
        } else {
            let mut form = form.build();
            form.error = Some(FormErrors::unmapped("User Name").to_string());
            return (form, DbPayloadBuilder::None);
        }

        (form.build(), DbPayloadBuilder::UserParams(payload_builder))
    }
}
