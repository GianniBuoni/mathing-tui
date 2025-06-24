use super::*;

impl Dialogue {
    pub fn delete_item(item: &StoreItem) -> Result<Self> {
        let message = format!("Confirm deletion of {}?", item.name);
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message)
            .with_req_type(RequestType::Delete)
            .with_from_type(AppArm::Items);
        let mut dialogue = dialogue.build()?;

        let Some(DbPayloadBuilder::ItemParams(params)) = &mut dialogue.payload
        else {
            let e = anyhow::Error::msg(
                "Mapping error: Form expects ItemParams payload.",
            );
            return Err(e);
        };
        params.item_id(ParamOption::new().map_value(item.id).to_owned());

        Ok(dialogue)
    }
}
