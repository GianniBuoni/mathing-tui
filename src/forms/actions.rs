use std::ops::Deref;

use super::*;

impl FormField<'_> {
    pub fn assign_index(&mut self, index: usize) {
        self.index = index
    }

    pub fn check_active(&mut self) {
        if self.index == *self.active_field.borrow().deref() {
            self.active = true
        }
    }
}
