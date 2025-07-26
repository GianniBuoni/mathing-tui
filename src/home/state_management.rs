use super::*;

impl Home {
    /// Get a reference to the current active table
    pub(super) fn try_get_current_table(
        &self,
    ) -> Result<&TableData, ComponentError> {
        self.components
            .get(self.component_tracker.inner())
            .ok_or(ComponentError::NoData)
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
        self.message = Some(Dialogue::error(err));
        self.mode = Mode::Insert;
    }
    pub(super) fn handle_paging(&mut self, action: Option<Action>) {
        if let Err(err) = (|| {
            let table = self
                .components
                .get_mut(self.component_tracker.inner())
                .ok_or(ComponentError::NoData)?;
            table.handle_action(action);

            if let Some(req) = table.get_req() {
                self.try_send(req)?
            }
            Aok(())
        })() {
            self.map_err(err);
        }
    }
}
