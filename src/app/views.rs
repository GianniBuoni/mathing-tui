use super::*;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub enum CurrentModel {
    #[default]
    Items,
    Receipt,
}

impl App {
    pub fn init_view(&mut self) {
        self.toggle_current_model();
    }

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

    fn toggle_current_model(&mut self) {
        if let Some(item) = self.models.get_mut(&mut self.current_model) {
            item.toggle();
        }
    }
}
