use super::*;

impl TableData {
    pub fn try_new_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        match table_type {
            AppArm::Items => Form::new_item().map(Some),
            AppArm::Users => Form::new_user().map(Some),
            _ => Ok(None),
        }
    }
    pub fn try_delete_form(&self) -> Result<Option<Dialogue>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        let Some(current_item) = self.items.get(self.table_index) else {
            return Ok(None);
        };

        match table_type {
            AppArm::Items => {
                let item = current_item.try_get_item()?;
                Dialogue::delete_item(item).map(Some)
            }
            AppArm::Users => {
                let user = current_item.try_get_user()?;
                Dialogue::delete_user(user).map(Some)
            }
            AppArm::Receipts => {
                let receipt = current_item.try_get_receipt()?;
                Dialogue::delete_reciept(receipt).map(Some)
            }
            AppArm::Totals => Ok(None),
        }
    }
    pub fn edit_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(ComponentError::not_found("Table type"))?;

        let item = self.try_get_active_item()?;

        match table_type {
            AppArm::Items => {
                let item = item.try_get_item()?;
                Form::edit_item(item).map(Some)
            }
            AppArm::Users => {
                let user = item.try_get_user()?;
                Form::edit_user(user).map(Some)
            }
            _ => Ok(None),
        }
    }
}
