use super::*;

impl Default for Tui {
    fn default() -> Self {
        Self::new()
    }
}

impl Tui {
    pub fn new() -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            terminal: ratatui::init(),
            event_tx,
            event_rx,
            task: tokio::spawn(async {}),
        }
    }

    pub fn start(mut self) -> Self {
        let event_loop = Self::event_loop(self.event_tx.clone());
        self.task = tokio::spawn(async {
            event_loop.await;
        });
        self
    }
}
