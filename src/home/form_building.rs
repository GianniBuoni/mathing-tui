use super::*;

impl Home {
    pub(super) fn enter_insert(&mut self) {
        match (|| {
            let form = self.try_get_current_table()?.try_new_form()?;
            Aok(form)
        })() {
            Ok(form) => {
                if let Some(form) = form {
                    self.form = Some(form);
                    self.mode = Mode::Insert;
                }
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn delete_selected(&mut self) {
        match (|| self.try_get_current_table()?.try_delete_form())() {
            Ok(log) => {
                if let Some(log) = log {
                    self.message = Some(log);
                    self.mode = Mode::Insert;
                }
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn edit_selected(&mut self) {
        match (|| {
            let table = self.try_get_current_table()?;
            let form = if let Some(AppArm::Receipts) = table.table_type {
                let (r, users) = self.edit_r_params()?;
                Form::edit_receipt(r, users).map(Some)?
            } else {
                table.edit_form()?
            };
            Aok(form)
        })() {
            Ok(form) => {
                if let Some(form) = form {
                    self.form = Some(form);
                    self.mode = Mode::Insert;
                }
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn handle_refresh(&mut self) {
        match Dialogue::refresh() {
            Ok(dialogue) => {
                self.message = Some(dialogue);
                self.mode = Mode::Insert;
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn handle_reset(&mut self) {
        match Dialogue::reset() {
            Ok(dialogue) => {
                self.message = Some(dialogue);
                self.mode = Mode::Insert;
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn handle_search(&mut self) {
        match Form::search_item() {
            Ok(form) => {
                self.form = Some(form);
                self.mode = Mode::Insert;
            }
            Err(err) => self.map_err(err),
        }
    }
}
