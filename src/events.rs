use std::{error::Error, sync::mpsc};

use ratatui::crossterm::event::{self, Event, KeyEvent};

pub fn send_key_event(
    rx: mpsc::Sender<KeyEvent>,
) -> Result<(), Box<dyn Error>> {
    loop {
        if let Event::Key(key_event) = event::read()? {
            rx.send(key_event)?
        }
    }
}
