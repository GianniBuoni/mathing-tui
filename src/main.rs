use std::{error::Error, sync::mpsc::channel, thread};

use mathing_tui::prelude::*;
use ratatui::crossterm::event::KeyEvent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let terminal = ratatui::init();
    let mut app = App::default();

    let (event_tx, event_rx) = channel::<KeyEvent>();

    thread::spawn(move || {
        if let Err(e) = send_key_event(event_tx) {
            ratatui::restore();
            eprint!("{e}");
        }
    });

    let app_result = app.run(terminal, event_rx);

    ratatui::restore();
    app_result
}
