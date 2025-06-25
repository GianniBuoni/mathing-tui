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
    ) -> Result<(&StoreJoinRow, Rc<[StoreUser]>), HomeErrors> {
        let (r, users) = self.build_r_form_params()?;

        let r = match r {
            DbTable::Item(_) => {
                return Err(HomeErrors::Mapping(
                    AppArm::Items,
                    AppArm::Receipts,
                ));
            }
            DbTable::User(_) => {
                return Err(HomeErrors::Mapping(
                    AppArm::Users,
                    AppArm::Receipts,
                ));
            }
            DbTable::Receipt(r) => r,
        };

        Ok((r, users))
    }

    // helper methods
    fn build_r_form_params(
        &self,
    ) -> Result<(&DbTable, Rc<[StoreUser]>), HomeErrors> {
        let table = self.check_for_table()?;
        // get active item of the table
        let Some(item) = table.get_active_item() else {
            return Err(HomeErrors::NoData);
        };
        // get user table
        let Some(table) = self.components.get(2) else {
            return Err(HomeErrors::not_found("Users"));
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
    fn new_r_params(
        &self,
    ) -> Result<(&StoreItem, Rc<[StoreUser]>), HomeErrors> {
        let (item, users) = self.build_r_form_params()?;

        let item = match item {
            DbTable::Item(i) => i,
            DbTable::User(_) => {
                return Err(HomeErrors::Mapping(AppArm::Users, AppArm::Items));
            }
            DbTable::Receipt(_) => {
                return Err(HomeErrors::Mapping(
                    AppArm::Receipts,
                    AppArm::Items,
                ));
            }
        };

        Ok((item, users))
    }
}
