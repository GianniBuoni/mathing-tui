use super::*;

impl Tui {
    pub fn builder() -> TuiBuilder {
        TuiBuilder::default()
    }
}

impl Default for TuiBuilder {
    fn default() -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(2);
        let (res_tx, res_rx) = tokio::sync::mpsc::channel(2);
        let (req_tx, req_rx) = tokio::sync::mpsc::unbounded_channel();

        Self {
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
        let event_loop = Tui::event_loop(self.event_tx);
        let db_loop = Tui::db_loop(self.req_rx, self.res_tx);

        Tui {
            terminal: ratatui::init(),
            _event_task: tokio::spawn(async {
                event_loop.await;
            }),
            _db_task: tokio::spawn(async {
                db_loop.await;
            }),
            event_rx: self.event_rx,
            res_rx: self.res_rx,
        }
    }
}
