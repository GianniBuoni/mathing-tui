use super::*;

impl<T> Component for Choice<T>
where
    T: Debug + Default + Copy,
{
    fn handle_action(&mut self, action: Option<Action>) {
        if !self.is_active() {
            return;
        }
        if let Some(Action::MakeSelection) = action {
            self.selected = !self.selected;
        };
    }
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.get_display().render(rect, frame.buffer_mut());
    }
    fn is_active(&self) -> bool {
        self.active_choice.inner() == self.index
    }
}
