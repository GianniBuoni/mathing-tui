use std::{error::Error, sync::mpsc};

use ratatui::crossterm::event::{self, Event, KeyEvent};

pub fn send_key_event(
    rx: mpsc::Sender<KeyEvent>,
) -> Result<(), Box<dyn Error>> {
    loop {
        match event::read()? {
            Event::Key(key_event) => rx.send(key_event)?,
            _ => (),
        }
    }
}
