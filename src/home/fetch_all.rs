use super::*;

impl Home {
    /// [`Home`]'s init method is responsible for making all the initial
    /// requests to the Db
    pub fn fetch_all(&mut self) {
        if let Err(err) = (|| {
            DbRequest::init()
                .into_iter()
                .try_for_each(|f| self.try_send(f))?;
            Aok(())
        })() {
            self.map_err(err);
        }
    }
}
