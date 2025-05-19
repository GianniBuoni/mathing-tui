use crate::prelude::*;
use std::fmt::Debug;

pub(crate) mod prelude {
    pub(crate) use super::{Component, ComponentBuilder};
}

pub trait ComponentBuilder<T, U>
where
    T: ComponentBuilder<T, U>,
    U: Component,
{
    fn add_component(self, component: Box<dyn Component>) -> T;
    fn build(self) -> U;
}

pub trait Component: Debug {
    fn handle_key_events(&mut self, key: KeyEvent) -> Option<Action>;
    fn update(&mut self, action: Option<Action>);
    fn draw(&mut self, frame: &mut Frame, rect: Rect);

    fn handle_events(&mut self, event: Option<Event>) -> Option<Action> {
        match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            _ => None,
        }
    }
}
