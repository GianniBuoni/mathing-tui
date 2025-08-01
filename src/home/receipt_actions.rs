use std::rc::Rc;

use super::*;

impl Home {
    pub(super) fn new_receipt(&mut self) {
        match (|| {
            let (item, users) = self.new_r_params()?;
            Form::new_receipt(item, users)
        })() {
            Ok(form) => {
                self.component_tracker.go_to(0);
                self.form = Some(form);
                self.mode = Mode::Insert;
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn edit_r_params(
        &self,
    ) -> Result<(&StoreJoinRow, Rc<[StoreUser]>)> {
        let (r, users) = self.build_r_form_params()?;
        let r = r.try_get_receipt()?;

        Ok((r, users))
    }

    // helper methods
    fn build_r_form_params(&self) -> Result<(&DbTable, Rc<[StoreUser]>)> {
        let item = self.try_get_current_table()?.try_get_active_item()?;

        let users = self
            .components
            .iter()
            .find(|f| f.table_type == Some(AppArm::Users))
            .ok_or(ComponentError::not_found("User table"))?
            .get_items()
            .iter()
            .filter_map(|f| match f {
                DbTable::User(u) => Some(u.to_owned()),
                _ => None,
            })
            .collect::<Rc<[StoreUser]>>();

        Ok((item, users))
    }
    fn new_r_params(&self) -> Result<(&StoreItem, Rc<[StoreUser]>)> {
        let (item, users) = self.build_r_form_params()?;
        let item = item.try_get_item()?;

        Ok((item, users))
    }
}
