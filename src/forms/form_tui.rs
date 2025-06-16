use super::*;

impl Component for FormTui {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::UserFrom(u) => u.draw(frame, rect),
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn handle_action(&mut self, action: Option<Action>) {
        match self {
            FormTui::UserFrom(u) => u.handle_action(action),
            FormTui::ItemForm(i) => i.handle_action(action),
            FormTui::ReceiptForm(r) => r.handle_action(action),
        }
    }

    fn handle_repsonse(&mut self, res: Option<&DbResponse>) {
        match res {
            Some(_) => {}
            None => {}
        }
    }
}

impl FormTui {
    pub fn submit(&mut self) -> Result<()> {
        match self {
            Self::UserFrom(u) => u.submit(),
            Self::ItemForm(i) => i.submit(),
            Self::ReceiptForm(r) => r.submit(),
        }
    }
    pub fn map_err(&mut self, err: Option<anyhow::Error>) {
        match self {
            Self::UserFrom(u) => u.map_err(err),
            Self::ItemForm(i) => i.map_err(err),
            Self::ReceiptForm(r) => r.map_err(err),
        }
    }
    pub fn get_req_type(&self) -> RequestType {
        match self {
            Self::UserFrom(u) => u.request_type,
            Self::ItemForm(i) => i.request_type,
            Self::ReceiptForm(r) => r.request_type,
        }
    }
}
