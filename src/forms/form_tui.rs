use super::*;

impl Component for FormTui {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::UserFrom(u) => u.draw(frame, rect),
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn update(
        &mut self,
        action: Option<Action>,
        response: Option<&DbResponse>,
    ) {
        match self {
            FormTui::UserFrom(u) => u.update(action, response),
            FormTui::ItemForm(i) => i.update(action, response),
            FormTui::ReceiptForm(r) => r.update(action, response),
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
