use super::*;

impl<T> Field for SelectionField<T>
where
    T: Debug + Default + Copy,
{
    fn submit(&self) -> Result<()> {
        self.choices
            .iter()
            .filter(|f| f.selected)
            .map(|f| f.value)
            .for_each(|value| self.values.borrow_mut().push(value));

        if !self.multiselect && self.values.borrow().len() > 1 {
            let e =
                FormErrors::validation("multi-select", "single select").into();
            return Err(e);
        }
        Ok(())
    }
    fn get_rect_height(&self) -> u16 {
        let Some(height) = self
            .choices
            .iter()
            .map(|f| f.get_rect_height())
            .reduce(|acc, f| acc + f)
        else {
            return Self::HEIGHT;
        };
        Self::HEIGHT + height
    }
}

impl<T> PluginInit for SelectionField<T>
where
    T: Debug + Default + Copy,
{
    fn init(&mut self, index: usize, tracker: ComponentTracker) {
        self.index = index;
        self.active_field = tracker;
    }
}

impl<T> PluginParent for SelectionField<T> where T: Debug + Default + Copy {}
