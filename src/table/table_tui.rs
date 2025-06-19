use super::*;

impl Component for TableTui {
    fn handle_action(&mut self, action: Option<Action>) {
        self.mut_inner(|f| f.handle_action(action));
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.mut_inner(|f| f.draw(frame, rect));
    }

    fn handle_response(&mut self, res: Option<&DbResponse>) {
        let Some(response) = res else {
            return;
        };

        match (self, &response.payload) {
            (TableTui::Items(i), DbPayload::Item(_) | DbPayload::Items(_)) => {
                i.handle_response(res)
            }
            (
                TableTui::Receipt(r),
                DbPayload::Receipt(_) | DbPayload::Receipts(_),
            ) => r.handle_response(res),
            (TableTui::Users(u), DbPayload::User(_) | DbPayload::Users(_)) => {
                u.handle_response(res)
            }
            _ => {}
        }
    }

    fn is_active(&self) -> bool {
        match self {
            TableTui::Items(i) => i.is_active(),
            TableTui::Receipt(r) => r.is_active(),
            TableTui::Users(u) => u.is_active(),
        }
    }
}

impl TableTui {
    pub fn new_form(&self) -> Option<FormTui> {
        match self {
            TableTui::Items(_) => Form::new_item(),
            TableTui::Users(_) => Form::new_user(),
            TableTui::Receipt(_) => None,
        }
    }
    pub fn get_inner<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&TableData) -> T,
    {
        match self {
            TableTui::Users(u) => f(u),
            TableTui::Items(i) => f(i),
            TableTui::Receipt(r) => f(r),
        }
    }
    pub fn mut_inner<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TableData),
    {
        match self {
            TableTui::Items(i) => f(i),
            TableTui::Users(u) => f(u),
            TableTui::Receipt(r) => f(r),
        }
    }
}
