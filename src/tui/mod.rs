use core::panic;

use crossterm::event::{EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;
use tokio::{
    sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::prelude::*;

pub mod prelude {
    pub use super::{Event, Tui, TuiBuilder};
}

mod builder;

pub enum Event {
    Init,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct Tui {
    pub terminal: DefaultTerminal,
    event_rx: Receiver<Event>,
    res_rx: Receiver<DbResponse>,
    _event_task: JoinHandle<()>,
    _db_task: JoinHandle<()>,
}

#[derive(Debug)]
pub struct TuiBuilder {
    event_tx: Sender<Event>,
    event_rx: Receiver<Event>,
    res_tx: Sender<DbResponse>,
    res_rx: Receiver<DbResponse>,
    pub req_tx: UnboundedSender<DbRequest>,
    req_rx: UnboundedReceiver<DbRequest>,
}

impl Tui {
    pub fn next_event(&mut self) -> Option<Event> {
        self.event_rx.try_recv().ok()
    }

    pub fn next_response(&mut self) -> Option<DbResponse> {
        self.res_rx.try_recv().ok()
    }

    async fn event_loop(event_tx: Sender<Event>) {
        let mut event_stream = EventStream::new();

        // if this fails, then it's likely a bug in the calling code
        event_tx
            .send(Event::Init)
            .await
            .expect("Failed to send Init event.");

        loop {
            let event = tokio::select! {
            crossterm_event = event_stream.next().fuse() => match crossterm_event {
                Some(Ok(event)) => match event {
                    CrosstermEvent::Key(key) if key.kind == KeyEventKind::Press => Event::Key(key),
                    _ => continue,
                }
                Some(Err(_)) => break,
                None => break,
                }
            };
            if event_tx.send(event).await.is_err() {
                break;
            }
        }
    }

    async fn db_loop(
        mut req_rx: UnboundedReceiver<DbRequest>,
        res_tx: Sender<DbResponse>,
    ) {
        let conn = match DbConn::try_get().await {
            Ok(c) => c,
            Err(_) => {
                let res = DbResponse::new()
                    .req_type(RequestType::GetAll)
                    .error(RequestError::Connection.to_string());
                res_tx
                    .send(res)
                    .await
                    .expect("Failed to send the Connection Error response.");
                return;
            }
        };

        loop {
            let res = tokio::select! {
                Some(req) = req_rx.recv() => handle_requests(req, conn).await,
                else => continue,
            };
            if res_tx.send(res).await.is_err() {
                break;
            }
        }
    }
}
