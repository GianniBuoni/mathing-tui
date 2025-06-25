use super::*;

impl<T> Field for SelectionField<T>
where
    T: Debug + Default + Copy,
{
    fn submit(&self) -> Result<()> {
        let choice_values = self
            .choices
            .iter()
            .filter(|f| f.selected)
            .map(|f| f.value)
            .collect::<Vec<T>>();

        // replace inner value with new collection
        self.values.replace(choice_values);
        // error if value is empty
        if self.values.borrow().is_empty() {
            let e = FormError::no_data("choices").into();
            return Err(e);
        }
        // error if value is longer than expected
        if !self.multiselect && self.values.borrow().len() > 1 {
            let e =
                FormError::validation("multi-select", "single select").into();
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
