use crate::prelude::*;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

pub mod prelude {
    pub use super::{Component, ComponentBuilder, Plugin};
}

pub trait ComponentBuilder<T>
where
    Self: Sized,
    T: Component,
{
    fn build(self) -> T;
    fn add_key_event_handler(
        &mut self,
        _keymap: HashMap<KeyEvent, Action>,
    ) -> &mut Self {
        todo!()
    }
}

pub trait Plugin: Component + Sized + 'static {
    fn add_to_app(self, app: &mut AppBuilder) {
        app.add_component(self);
    }
}

pub trait Component: Debug {
    fn update(&mut self, action: Option<Action>, response: Option<&DbResponse>);
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

    fn init(&mut self, _index: usize, _tracker: Rc<RefCell<usize>>) {}

    fn is_active(&self) -> bool {
        false
    }
}
