use std::error::Error;

use crossterm::event::KeyEvent;
use futures::channel::mpsc::UnboundedSender;

use crate::prelude::*;

pub trait ComponentBuilder<T, U>
where
    T: ComponentBuilder<T, U>,
    U: Component,
{
    fn add_action_handler(&mut self, tx: UnboundedSender<Action>) -> T;
    fn build(&self) -> U;
}

pub trait Component {
    fn handle_key_events(
        &mut self,
        key: KeyEvent,
    ) -> Result<Option<Action>, Box<dyn Error>>;

    fn handle_events(
        &mut self,
        event: Option<Event>,
    ) -> Result<Option<Action>, Box<dyn Error>> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            _ => None,
        };
        Ok(r)
    }

    fn update(
        &mut self,
        action: Action,
    ) -> Result<Option<Action>, Box<dyn Error>>;
    fn draw(&mut self, frame: Frame, rect: Rect) -> std::io::Result<()>;
}
