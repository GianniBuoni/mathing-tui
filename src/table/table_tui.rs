use super::*;

impl Component for TableTui {
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        match self {
            TableTui::Items(i) => i.init(index, tracker),
            TableTui::Receipt(r) => r.init(index, tracker),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self {
            TableTui::Items(i) => i.update(action),
            TableTui::Receipt(r) => r.update(action),
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            TableTui::Items(i) => i.draw(frame, rect),
            TableTui::Receipt(r) => r.draw(frame, rect),
        }
    }
}

impl TableTui {
    pub fn is_active(&self) -> bool {
        match self {
            TableTui::Items(i) => i.active,
            TableTui::Receipt(i) => i.active,
        }
    }
}
