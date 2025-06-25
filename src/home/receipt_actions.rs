use std::rc::Rc;

use super::*;

impl Home {
    pub(super) fn new_receipt(&mut self) {
        match (|| {
            anyhow::Ok::<Form>({
                let (item, users) = self.new_r_params()?;
                Form::new_receipt(item, users)?
            })
        })() {
            Ok(form) => {
                self.component_tracker.go_to(0);
                self.form = Some(form);
                self.mode = Mode::Insert;
            }
            Err(err) => self.map_err(err),
        }
    }
    pub(super) fn edit_r_parms(
        &self,
    ) -> Result<(&StoreJoinRow, Rc<[StoreUser]>)> {
        let (r, users) = self.build_r_form_params()?;
        let r = r.get_receipt()?;

        Ok((r, users))
    }

    // helper methods
    fn build_r_form_params(
        &self,
    ) -> Result<(&DbTable, Rc<[StoreUser]>), ComponentError> {
        let table = self.check_for_table()?;
        // get active item of the table
        let Some(item) = table.get_active_item() else {
            return Err(ComponentError::NoData);
        };
        // get user table
        let Some(table) = self.components.get(2) else {
            return Err(ComponentError::not_found("Users"));
        };
        let users = table.get_items();
        let users = users
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
        let item = item.get_item()?;

        Ok((item, users))
    }
}
