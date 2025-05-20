use crossterm::event::{EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};

use crate::prelude::*;

pub mod prelude {
    pub use super::{Event, Tui};
}

mod builder;

pub enum Event {
    Init,
    Quit,
    Error,
    Key(KeyEvent),
}

pub struct Tui {
    pub terminal: DefaultTerminal,
    event_rx: tokio::sync::mpsc::UnboundedReceiver<Event>,
    event_tx: tokio::sync::mpsc::UnboundedSender<Event>,
    task: JoinHandle<()>,
}

impl Tui {
    pub async fn next_event(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }

    async fn event_loop(event_tx: UnboundedSender<Event>) {
        // if this fails, then it's likely a bug in the calling code
        let mut event_stream = EventStream::new();
        event_tx
            .send(Event::Init)
            .expect("Failed to send Init event");
        loop {
            let event = tokio::select! {
            crossterm_event = event_stream.next().fuse() => match crossterm_event {
                Some(Ok(event)) => match event {
                    CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
                    _ => continue,
                }
                Some(Err(_)) => Event::Error,
                None => break,
                }
            };
            if event_tx.send(event).is_err() {
                break;
            }
        }
    }
}
