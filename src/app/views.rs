use super::*;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CurrentModel {
    #[default]
    Items,
    Receipt,
}

impl App {
    pub fn cycle_view(&mut self) {
        match self.current_model {
            CurrentModel::Items => {
                self.toggle_current_model();
                self.current_model = CurrentModel::Receipt;
                self.toggle_current_model();
            }
            CurrentModel::Receipt => {
                self.toggle_current_model();
                self.current_model = CurrentModel::Items;
                self.toggle_current_model();
            }
        }
    }
}
