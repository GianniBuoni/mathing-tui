use std::error::Error;

use super::*;

pub trait Form: WidgetRef + Debug {
    fn cursor_pos(&self) -> Result<Position, Box<dyn Error>>;
    fn handle_event(&self, event: &event::KeyEvent);
}

impl<T> Form for FormWidget<'_, T>
where
    T: Default + Debug,
{
    fn cursor_pos(&self) -> Result<Position, Box<dyn Error>> {
        if let Some(field) = self.inputs.get(self.active_field) {
            Ok(field.get_cursor()?)
        } else {
            return Err(
                "Could not get an active field from current from".into()
            );
        }
    }
    fn handle_event(&self, _event: &event::KeyEvent) {
        todo!()
    }
}
