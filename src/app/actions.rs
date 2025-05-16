use super::*;

// TODO: move to db, connect to FromActions struct
pub struct QueryRequest;

pub enum Action {
    Quit,
    SwitchPane,
    TableNavigateDown,
    TableNavigateUp,
    Query(QueryRequest),
}

impl App {
    pub fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::Quit) => {
                self.should_exit = true;
            }
            Some(Action::SwitchPane) => {
                self.cycle_view();
            }
            Some(Action::Query(_q_params)) => {
                todo!()
            }
            Some(Action::TableNavigateDown) => {
                if let Some(model) = self.models.get_mut(&self.current_model) {
                    model.next_row();
                }
            }
            Some(Action::TableNavigateUp) => {
                if let Some(model) = self.models.get_mut(&self.current_model) {
                    model.prev_row();
                }
            }
            None => {}
        }
    }
}
