use super::*;

impl Component for App {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let _ = frame;
        let _ = rect;
        todo!()
    }
    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Key(_)) => self.component.handle_events(event),
            Some(Event::Init) => {
                self.component.fetch_all();
                None
            }
            None => None,
        }
    }
    fn handle_action(&mut self, action: Option<Action>) {
        match action {
            Some(Action::Quit) => {
                self.should_exit = true;
            }
            Some(_) => {
                self.component.handle_action(action);
            }
            None => {}
        }
    }
    fn handle_response(&mut self, res: Option<&DbResponse>) {
        self.component.handle_response(res);
    }
}
