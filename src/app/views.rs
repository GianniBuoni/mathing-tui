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
        // TODO: move to struct
        todo!()
    }

    fn toggle_current_model(&mut self) {
        // TODO: move to Home struct
    }
}
