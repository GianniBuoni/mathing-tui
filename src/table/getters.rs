use super::*;

impl TableData {
    /// Calculates the offset for a GetAll request
    pub fn get_items(&self) -> Rc<[DbTable]> {
        self.items.clone().into()
    }
    pub fn try_get_active_item(&self) -> Result<&DbTable> {
        Ok(self
            .items
            .get(self.table_index)
            .ok_or(ComponentError::NoData)?)
    }
    // Public setters
    pub fn set_search(&mut self, search_term: impl Into<Rc<str>>) {
        self.last_search = Some(search_term.into())
    }
}
