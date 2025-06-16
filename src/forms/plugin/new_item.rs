use super::*;

impl Form {
    pub fn new_item_form() -> (Self, Option<DbPayloadBuilder>) {
        let payload_builder =
            ItemParamsBuilder::default().item_name("").item_price(0.);
        let mut form = Self::builder();

        form.add_title("New Item")
            .add_rect(Self::form_rect(Self::TWO_FIELD_H))
            .add_request_type(RequestType::Post);

        if let Err(err) = form
            .try_map_input::<String>(&payload_builder.item_name, "Item Name")
        {
            return (form.build_with_error(err), None);
        }

        if let Err(err) =
            form.try_map_input(&payload_builder.item_price, "Item Price")
        {
            return (form.build_with_error(err), None);
        }

        (
            form.build(),
            Some(DbPayloadBuilder::ItemParams(payload_builder)),
        )
    }
}
