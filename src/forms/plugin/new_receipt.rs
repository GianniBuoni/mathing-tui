use super::*;

impl Form {
    pub fn new_receipt(
        item: &StoreItem,
    ) -> (Option<FormTui>, Option<DbPayloadBuilder>) {
        let payload_builder =
            JoinParamsBuilder::default().item_id(item.id).item_qty(0);
        let mut form = Self::builder();

        form.add_title("New Receipt")
            .add_rect(Self::form_rect(Self::THREE_FIELD_H))
            .add_request_type(RequestType::Post);

        // map fields
        if let Err(err) =
            form.try_map_input(&payload_builder.item_qty, "Item Qty")
        {
            return (
                Some(FormTui::ReceiptForm(form.build_with_error(err))),
                None,
            );
        }

        (
            Some(FormTui::ReceiptForm(form.build())),
            Some(DbPayloadBuilder::ReceiptParams(payload_builder)),
        )
    }
}
