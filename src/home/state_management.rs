use super::*;

impl Home {
    /// Check wether the main component has table components associated with
    /// the main comopnents active index. Errors out if none exists.
    pub(super) fn check_for_table(&self) -> Result<&TableData, HomeErrors> {
        let Some(table) = self.components.get(self.component_tracker.inner())
        else {
            return Err(HomeErrors::NoData);
        };
        Ok(table)
    }
    /// Resets main component's form and message fields to None and then resets
    /// the component to Normal mode.
    pub(super) fn reset_mode(&mut self) {
        self.form = None;
        self.message = None;
        self.mode = Mode::Normal
    }
    pub(super) fn cycle_active(&mut self, add: i32) {
        if self.components.is_empty() {
            return;
        }
        let max = self.components.len() - 1;
        let new_index = self.component_tracker.inner() as i32 + add;

        match new_index {
            int if int > max as i32 => self.component_tracker.go_to(0),
            int if int < 0 => self.component_tracker.go_to(max),
            _ => self.component_tracker.go_to(new_index as usize),
        }
    }
    /// Report back to main component if the current dialogue box (if there
    /// is one) is an error message.
    pub(super) fn is_error(&self) -> bool {
        let Some(message) = self.message.as_ref() else {
            return false;
        };
        message.is_error()
    }
    /// Map any error into a dialogue popup.
    pub(super) fn map_err(&mut self, err: impl Display) {
        if let Some(form) = self.form.as_mut() {
            form.map_err(err);
            self.mode = Mode::Insert;
            return;
        }
        self.message = Some(Dialogue::error(err))
    }
}
