use anyhow::Error;

use super::*;

impl TableData {
    // public methods
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }
    pub fn new_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(Error::msg("Table is missing a form type."))?;

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
            .ok_or(Error::msg("Table if missing a form type."))?;

        let Some(current_item) = self.items.get(self.table_index) else {
            return Ok(None);
        };

        match table_type {
            AppArm::Items => {
                let DbTable::Item(item) = current_item else {
                    // TODO: make error?
                    return Ok(None);
                };
                Dialogue::delete_item(item).map(Some)
            }
            AppArm::Users => {
                let DbTable::User(user) = current_item else {
                    return Ok(None);
                };
                Dialogue::delete_user(user).map(Some)
            }
            AppArm::Receipts => {
                let DbTable::Receipt(receipt) = current_item else {
                    return Ok(None);
                };
                Dialogue::delete_reciept(receipt).map(Some)
            }
        }
    }
    pub fn edit_form(&self) -> Result<Option<Form>> {
        let table_type = self
            .table_type
            .as_ref()
            .ok_or(Error::msg("Table if missing a form type."))?;

        let Some(item) = self.get_active_item() else {
            return Ok(None);
        };

        match table_type {
            AppArm::Items => {
                let DbTable::Item(item) = item else {
                    return Ok(None);
                };
                Form::edit_item(item).map(Some)
            }
            AppArm::Users => {
                let DbTable::User(user) = item else {
                    return Ok(None);
                };
                Form::edit_user(user).map(Some)
            }
            AppArm::Receipts => Ok(None),
        }
    }

    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn get_active_item(&self) -> Option<&DbTable> {
        self.items.get(self.table_index)
    }

    // private methods
    pub(super) fn add_items(&mut self, items: Vec<DbTable>) {
        match items.len() {
            1 => {
                self.items.push(items.first().unwrap().clone());
            }
            _ => self.items = items,
        }
    }
    fn max(&self) -> usize {
        self.items.len() - 1
    }
    pub(super) fn next_row(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }
    pub(super) fn prev_row(&mut self) {
        if !self.is_active() || self.items.is_empty() {
            return;
        }
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }
}
