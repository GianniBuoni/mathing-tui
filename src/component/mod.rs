use std::{error::Error, fmt::Debug};

use crossterm::event::KeyEvent;
use futures::channel::mpsc::UnboundedSender;

use crate::prelude::*;

pub(crate) mod prelude {
    pub(crate) use super::home::Home;
}

mod home;

pub trait ComponentBuilder<T, U>
where
    T: ComponentBuilder<T, U>,
    U: Component,
{
    fn add_action_handler(self, tx: UnboundedSender<Action>) -> T;
    fn build(&self) -> U;
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
