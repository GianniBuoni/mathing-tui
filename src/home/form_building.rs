use super::*;

impl Home {
    pub(super) fn set_form(&mut self, form: Result<Option<Form>>) {
        match form {
            Ok(f) => {
                if f.is_some() {
                    self.form = f;
                    self.mode = Mode::Insert;
                }
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn set_msg(&mut self, dialogue: Result<Option<Dialogue>>) {
        match dialogue {
            Ok(d) => {
                if d.is_some() {
                    self.message = d;
                    self.mode = Mode::Insert;
                }
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn enter_insert(&mut self) {
        let form = || self.try_get_current_table()?.try_new_form();
        self.set_form(form());
    }
    pub(super) fn delete_selected(&mut self) {
        let msg = || self.try_get_current_table()?.try_delete_form();
        self.set_msg(msg());
    }
    pub(super) fn edit_selected(&mut self) {
        let form = || {
            let active_table = self.try_get_current_table()?;
            if let Some(AppArm::Receipts) = active_table.table_type {
                let (r, users) = self.edit_r_params()?;
                Form::edit_receipt(r, users).map(Some)
            } else {
                active_table.edit_form()
            }
        };
        self.set_form(form());
    }
}
