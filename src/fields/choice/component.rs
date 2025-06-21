use super::*;

impl<T> Component for Choice<T>
where
    T: Debug + Default + Copy,
{
    fn handle_action(&mut self, action: Option<Action>) {
        let Some(action) = action else {
            return;
        };
        let Action::HandleInput(key_event) = action else {
            return;
        };
        if let KeyCode::Char(' ') = key_event.code {
            self.selected = !self.selected;
        }
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.get_display().render(rect, frame.buffer_mut());
    }
    fn is_active(&self) -> bool {
        self.active_choice.inner() == self.index
    }
}
