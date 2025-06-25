use super::*;

impl TableData {
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn get_active_item(&self) -> Option<&DbTable> {
        self.items.get(self.table_index)
    }
}
