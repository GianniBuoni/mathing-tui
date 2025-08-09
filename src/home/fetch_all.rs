use super::*;

impl Home {
    /// [`Home`]'s init method is responsible for making all the initial
    /// requests to the Db
    pub fn fetch_all(&mut self) {
        if let Err(err) = self.components.iter().try_for_each(|f| {
            let get = f.get_req().ok_or_else(|| {
                let message = "Couldt't initialize a get req for table.";
                Error::msg(message)
            })?;
            let count = f.count().ok_or_else(|| {
                let message = "Couldn't initialize a count req for table.";
                Error::msg(message)
            })?;
            self.try_send(get)?;
            self.try_send(count)
        }) {
            self.map_err(err);
        }
    }
}
