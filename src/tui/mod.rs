use crossterm::event::{EventStream, KeyEventKind};
use futures::{FutureExt, StreamExt};
use ratatui::DefaultTerminal;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::prelude::*;

pub mod prelude {
    pub use super::{DbRequest, Event, Tui};
}

mod builder;

pub enum Event {
    Init,
    Quit,
    Error,
    Key(KeyEvent),
}

pub struct DbResponse {
    req_type: RequestType,
    payload: DbPayload,
    error: Option<String>,
}

pub struct DbRequest {
    req_type: RequestType,
    payload: DbPayload,
}

pub enum DbPayload {
    ItemParams(ItemParams),
    ReceiptParamss(ReceiptParams),
    Item(StoreItem),
    Receipt(StoreJoinRow),
}

#[derive(Clone, Copy)]
pub enum RequestType {
    Get,
    Post,
    Update,
    Delete,
}

pub struct Tui {
    pub terminal: DefaultTerminal,
    event_rx: UnboundedReceiver<Event>,
    event_tx: UnboundedSender<Event>,
    res_rx: UnboundedReceiver<DbResponse>,
    res_tx: UnboundedSender<DbResponse>,
    req_tx: UnboundedSender<DbRequest>,
    event_task: JoinHandle<()>,
    db_task: JoinHandle<()>,
}

impl Tui {
    pub fn next_event(&mut self) -> Option<Event> {
        match self.event_rx.try_recv() {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }

    pub fn next_response(&mut self) -> Option<DbResponse> {
        match self.res_rx.try_recv() {
            Ok(_) => None,
            Err(_) => None,
        }
    }

    async fn event_loop(event_tx: UnboundedSender<Event>) {
        let mut event_stream = EventStream::new();

        // if this fails, then it's likely a bug in the calling code
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

    async fn db_loop(
        mut req_rx: UnboundedReceiver<DbRequest>,
        res_tx: UnboundedSender<DbResponse>,
    ) {
        // inital fetch should go here

        loop {
            let res = tokio::select! {
                Some(req) = req_rx.recv() => match (req.req_type, req.payload){
                    (RequestType::Get, DbPayload::Item(i)) => {
                        DbResponse {
                            req_type: req.req_type,
                            payload: DbPayload::Item(i),
                            error: None,
                        }
                    }
                    _ => break,
                },
                else => break,
            };
            if res_tx.send(res).is_err() {
                break;
            }
        }
    }
}
