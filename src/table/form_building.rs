use super::*;

impl TableData {
    pub fn new_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        match table_type {
            AppArm::Items => Form::new_item().map(Some),
            AppArm::Users => Form::new_user().map(Some),
            AppArm::Receipts => Ok(None),
        }
    }
    pub fn delete_form(&self) -> Result<Option<Dialogue>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        let Some(current_item) = self.items.get(self.table_index) else {
            return Ok(None);
        };

        match table_type {
            AppArm::Items => {
                let item = current_item.get_item()?;
                Dialogue::delete_item(item).map(Some)
            }
            AppArm::Users => {
                let user = current_item.get_user()?;
                Dialogue::delete_user(user).map(Some)
            }
            AppArm::Receipts => {
                let receipt = current_item.get_receipt()?;
                Dialogue::delete_reciept(receipt).map(Some)
            }
        }
    }
    pub fn edit_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        let Some(item) = self.get_active_item() else {
            return Ok(None);
        };

        match table_type {
            AppArm::Items => {
                let item = item.get_item()?;
                Form::edit_item(item).map(Some)
            }
            AppArm::Users => {
                let user = item.get_user()?;
                Form::edit_user(user).map(Some)
            }
            AppArm::Receipts => Ok(None),
        }
    }
}
