use crate::prelude::*;
use std::{collections::HashMap, fmt::Debug};

pub mod prelude {
    pub use super::{Component, ComponentBuilder};
}

pub trait ComponentBuilder<T>
where
    Self: Sized,
    T: Component,
{
    fn build(self) -> T;
    fn add_key_event_handler(self, _keymap: HashMap<KeyEvent, Action>) -> Self {
        todo!()
    }
}

pub trait Component: Debug {
    fn update(&mut self, action: Option<Action>);
    fn draw(&mut self, frame: &mut Frame, rect: Rect);

    fn handle_key_events(&self, _key: KeyEvent) -> Option<Action> {
        todo!();
    }

    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            _ => None,
        }
    }

    fn init(&mut self) {}
}
