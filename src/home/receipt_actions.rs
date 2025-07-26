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
    pub(super) fn edit_r_params(
        &self,
    ) -> Result<(&StoreJoinRow, Rc<[StoreUser]>)> {
        let (r, users) = self.build_r_form_params()?;
        let r = r.try_get_receipt()?;

        Ok((r, users))
    }
    pub(super) fn try_subtract_store_total(&self) -> Result<()> {
        let current_r = self
            .try_get_current_table()?
            .try_get_active_item()?
            .try_get_receipt()?;

        StoreTotal::try_get()?
            .lock()
            .unwrap()
            .subtract(current_r.try_calc()?);

        Ok(())
    }

    // helper methods
    fn build_r_form_params(&self) -> Result<(&DbTable, Rc<[StoreUser]>)> {
        let item = self.try_get_current_table()?.try_get_active_item()?;

        // get user table
        let Some(table) = self.components.get(2) else {
            return Err(ComponentError::not_found("Users").into());
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
        let item = item.try_get_item()?;

        Ok((item, users))
    }
}
