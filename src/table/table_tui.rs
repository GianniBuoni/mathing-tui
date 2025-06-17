use super::*;

impl Component for TableTui {
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        match self {
            TableTui::Items(i) => i.init(index, tracker),
            TableTui::Receipt(r) => r.init(index, tracker),
            TableTui::Users(u) => u.init(index, tracker),
        }
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match self {
            TableTui::Items(i) => i.handle_action(action),
            TableTui::Receipt(r) => r.handle_action(action),
            TableTui::Users(u) => u.handle_action(action),
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            TableTui::Items(i) => i.draw(frame, rect),
            TableTui::Receipt(r) => r.draw(frame, rect),
            TableTui::Users(u) => u.draw(frame, rect),
        }
    }

    fn handle_repsonse(&mut self, res: Option<&DbResponse>) {
        match self {
            TableTui::Items(i) => i.handle_repsonse(res),
            TableTui::Receipt(r) => r.handle_repsonse(res),
            TableTui::Users(u) => u.handle_repsonse(res),
        }
    }

    fn is_active(&self) -> bool {
        match self {
            TableTui::Items(i) => i.active,
            TableTui::Receipt(r) => r.active,
            TableTui::Users(u) => u.active,
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
}
