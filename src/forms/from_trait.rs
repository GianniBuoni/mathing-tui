use super::*;

pub trait Form: WidgetRef + Debug {
    fn submit(&self);
    fn cursor_pos(&self) -> Position;
    fn handle_event(&self, event: &event::KeyEvent);
}

impl<T> Form for FormWidget<'_, T>
where
    T: Default + Debug,
{
    fn submit(&self) {
        todo!()
    }
    fn cursor_pos(&self) -> Position {
        todo!()
    }
    fn handle_event(&self, _event: &event::KeyEvent) {
        todo!()
    }
}
