use crate::prelude::*;
use anyhow::Result;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub mod prelude {
    pub use super::{Component, ComponentBuilder, ComponentTracker};
}

#[derive(Debug, Default, Clone)]
pub struct ComponentTracker(Rc<RefCell<usize>>);

impl ComponentTracker {
    pub fn go_to(&self, index: usize) {
        *self.0.borrow_mut() = index;
    }
    pub fn inner(&self) -> usize {
        *self.0.borrow()
    }
}

pub trait ComponentBuilder
where
    Self: Sized,
{
    type Output: Component;

    fn build(self) -> Result<Self::Output>;
}

pub trait Component: Debug {
    // required methods
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    fn handle_action(&mut self, action: Option<Action>);

    // provided methods
    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            _ => None,
        }
    }
    // optional methods
    fn handle_response(&mut self, res: Option<&DbResponse>) -> Result<()> {
        let _ = res;
        todo!()
    }
    fn handle_key_events(&self, key: KeyEvent) -> Option<Action> {
        let _ = key;
        todo!();
    }
    fn is_active(&self) -> bool {
        todo!();
    }
}
