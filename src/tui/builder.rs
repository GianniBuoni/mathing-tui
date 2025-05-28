use super::*;

pub struct TuiBuilder {
    event_tx: UnboundedSender<Event>,
    event_rx: UnboundedReceiver<Event>,
    res_tx: UnboundedSender<DbResponse>,
    res_rx: UnboundedReceiver<DbResponse>,
    req_tx: UnboundedSender<DbRequest>,
    req_rx: UnboundedReceiver<DbRequest>,
}

impl Tui {
    pub fn builder() -> TuiBuilder {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        let (res_tx, res_rx) = tokio::sync::mpsc::unbounded_channel();
        let (req_tx, req_rx) = tokio::sync::mpsc::unbounded_channel();

        TuiBuilder {
            event_tx,
            event_rx,
            res_tx,
            res_rx,
            req_tx,
            req_rx,
        }
    }
}

impl TuiBuilder {
    pub fn build(self) -> Tui {
        let event_loop = Tui::event_loop(self.event_tx.clone());
        let db_loop = Tui::db_loop(self.req_rx, self.res_tx.clone());

        Tui {
            terminal: ratatui::init(),
            event_task: tokio::spawn(async {
                event_loop.await;
            }),
            db_task: tokio::spawn(async {
                db_loop.await;
            }),
            event_tx: self.event_tx,
            event_rx: self.event_rx,
            res_tx: self.res_tx,
            res_rx: self.res_rx,
            req_tx: self.req_tx,
        }
    }
}
