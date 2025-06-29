use super::*;

impl Home {
    /// [`Home`]'s init method is responsible for making all the initial
    /// requests to the Db
    pub fn fetch_all(&mut self) {
        if let Err(err) = (|| -> Result<()> {
            let tx = self
                .req_tx
                .clone()
                .ok_or(ComponentError::not_found("req_tx"))?;
            DbRequest::init().into_iter().try_for_each(|f| tx.send(f))?;
            Ok(())
        })() {
            self.map_err(err);
        }
    }
}
