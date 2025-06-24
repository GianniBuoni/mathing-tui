use super::*;

impl TableData {
    // public methods
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }
    pub fn new_form(&self) -> Option<Result<Form>> {
        let Some(table_type) = &self.table_type else {
            return None;
        };
        match table_type {
            AppArm::Items => Some(Form::new_item()),
            AppArm::Users => Some(Form::new_user()),
            AppArm::Receipts => None,
        }
    }
    pub fn delete_form(&self) -> Option<Result<Dialogue>> {
        let Some(table_type) = &self.table_type else {
            return None;
        };
        let Some(current_item) = self.items.get(self.table_index) else {
            return None;
        };
        match table_type {
            AppArm::Items => {
                let DbTable::Item(item) = current_item else {
                    return None;
                };
                Some(Dialogue::delete_item(item))
            }
            AppArm::Users => None,
            AppArm::Receipts => None,
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
        if !self.is_active() {
            return;
        }
        if self.items.is_empty() {
            return;
        }
        if self.table_index < self.max() {
            self.table_index += 1
        } else {
            self.table_index = 0
        }
    }
    pub(super) fn prev_row(&mut self) {
        if !self.is_active() {
            return;
        }
        if self.items.is_empty() {
            return;
        }
        if self.table_index > 0 {
            self.table_index -= 1
        } else {
            self.table_index = self.max()
        }
    }
}
