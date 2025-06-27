use super::*;

impl TableData {
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn try_get_active_item(&self) -> Result<&DbTable> {
        Ok(self
            .items
            .get(self.table_index)
            .ok_or(ComponentError::NoData)?)
    }
}
