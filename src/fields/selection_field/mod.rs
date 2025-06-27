use super::*;
use crate::forms::FormBuilder;

mod builder;
mod component;
mod field;
mod plugin;

impl<T> SelectionField<T>
where
    T: Debug + Default + Copy,
{
    const HEIGHT: u16 = 4;

    pub fn builder() -> SelectionBuilder<T> {
        SelectionBuilder::<T>::default()
    }

    fn max(&self) -> usize {
        self.choices.len() - 1
    }
    fn cycle_next(&self, add: i32) {
        if self.choices.is_empty() {
            return;
        }

        let next_index = self.active_choice.inner() as i32 + add;
        match next_index {
            int if int < 0 => self.active_choice.go_to(self.max()),
            int if int > self.max() as i32 => self.active_choice.go_to(0),
            _ => self.active_choice.go_to(next_index as usize),
        }
    }
}
