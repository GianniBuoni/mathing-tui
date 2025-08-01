use crate::config::HelpMap;

use super::*;

impl Dialogue {
    pub fn delete_item(item: &StoreItem) -> Result<Self> {
        let message = format!("Confirm deletion of {}?", item.name);
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message, Color::Reset)
            .with_req_type(RequestType::Delete)
            .with_from_type(AppArm::Items);
        let mut dialogue = dialogue.build()?;

        let Some(DbPayloadBuilder::ItemParams(params)) =
            dialogue.payload.as_mut()
        else {
            let e = anyhow::Error::msg(
                "Mapping error: Form expects ItemParams payload.",
            );
            return Err(e);
        };
        params.with_item_id(ParamOption::new().map_value(item.id).to_owned());

        Ok(dialogue)
    }

    pub fn delete_user(user: &StoreUser) -> Result<Self> {
        let message = format!("Confirm deletion of {}?", user.name);
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message, Color::Reset)
            .with_req_type(RequestType::Delete)
            .with_from_type(AppArm::Users);
        let mut dialogue = dialogue.build()?;

        let Some(DbPayloadBuilder::UserParams(params)) =
            dialogue.payload.as_mut()
        else {
            let e = anyhow::Error::msg(
                "Mapping error: Form expects ItemParams payload.",
            );
            return Err(e);
        };
        params.with_user_id(ParamOption::new().map_value(user.id).to_owned());

        Ok(dialogue)
    }

    pub fn delete_reciept(receipt: &StoreJoinRow) -> Result<Self> {
        let message = format!("Confirm deletion of {}?", receipt.item_name);
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message, Color::Reset)
            .with_req_type(RequestType::Delete)
            .with_from_type(AppArm::Receipts);
        let mut dialogue = dialogue.build()?;

        let Some(DbPayloadBuilder::ReceiptParams(params)) =
            dialogue.payload.as_mut()
        else {
            let e = anyhow::Error::msg(
                "Mapping error: Form expects ItemParams payload.",
            );
            return Err(e);
        };
        params.with_r_id(
            ParamOption::new().map_value(receipt.receipt_id).to_owned(),
        );

        Ok(dialogue)
    }

    pub fn refresh() -> Result<Self> {
        let message = "Confirm refetch of all table data?";
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message, Color::Reset)
            .with_req_type(RequestType::Update)
            .with_from_type(AppArm::Totals);

        dialogue.build()
    }

    pub fn reset() -> Result<Self> {
        let message = "Confirm receipt reset?";
        let mut dialogue = Self::builder();

        dialogue
            .with_message(message, Color::Reset)
            .with_req_type(RequestType::Reset)
            .with_from_type(AppArm::Receipts);

        dialogue.build()
    }

    pub fn help() -> Result<Self> {
        let heading = AppColors::ACTIVE.ground;
        let lines = HelpMap::get_lines();
        let dirs = ConfigDirs::get()?;

        let mut dialogue = Self::builder();
        dialogue
            .with_message("[Keymap (key code: description)]", heading)
            .with_message("\n", Color::Reset);

        lines.iter().enumerate().for_each(|(index, line)| {
            let color = match index % 2 {
                0 => Color::Reset,
                _ => Color::DarkGray,
            };
            dialogue.with_message(line, color);
        });

        let keymap_dir = format!("Keymap: {}", dirs.keymap);
        let db_dir = format!("Database: {}", dirs.db);
        dialogue
            .with_message("\n", Color::Reset)
            .with_message("[Config files]", heading)
            .with_message("\n", Color::Reset)
            .with_message(keymap_dir, Color::Reset)
            .with_message(db_dir, Color::Reset);

        let dialogue = dialogue.build()?;
        Ok(dialogue)
    }
}
