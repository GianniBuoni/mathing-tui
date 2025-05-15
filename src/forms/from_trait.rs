use std::error::Error;

use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};

use super::*;

pub trait Form: WidgetRef + Debug {
    fn cursor_pos(&self) -> Result<Position, Box<dyn Error>>;
    fn handle_event(&mut self, event: &event::KeyEvent);
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
    fn handle_event(&mut self, event: &event::KeyEvent) {
        match (event.code, event.modifiers) {
            (KeyCode::Tab, KeyModifiers::NONE) => {
                self.next_feild();
            }
            (KeyCode::Tab, KeyModifiers::SHIFT) => {
                self.prev_feild();
            }
            _ => {
                if let Some(form) = self.inputs.get_mut(self.active_field) {
                    form.input.handle_event(&event::Event::Key(*event));
                }
            }
        }
    }
}
