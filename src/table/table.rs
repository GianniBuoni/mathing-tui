use super::*;

impl Component for TableTui<'_> {
    fn init(&mut self) {
        match self {
            TableTui::Items(i) => i.init(),
            TableTui::Receipt(r) => r.init(),
        }
    }

    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match self {
            TableTui::Items(i) => i.handle_events(event),
            TableTui::Receipt(r) => r.handle_events(event),
        }
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action> {
        match self {
            TableTui::Items(i) => i.handle_key_events(key),
            TableTui::Receipt(r) => r.handle_key_events(key),
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

impl TableTui<'_> {
    pub fn add_tracker(&mut self, tracker: Rc<RefCell<usize>>) {
        match self {
            TableTui::Items(i) => {
                i.tracker = tracker;
            }
            TableTui::Receipt(r) => {
                r.tracker = tracker;
            }
        }
    }
    pub fn is_active(&self) -> bool {
        match self {
            TableTui::Items(i) => i.active,
            TableTui::Receipt(i) => i.active,
        }
    }
}
