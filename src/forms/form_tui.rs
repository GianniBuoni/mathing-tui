use super::*;

impl Component for FormTui {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::UserFrom(u) => u.draw(frame, rect),
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self {
            FormTui::UserFrom(u) => u.update(action),
            FormTui::ItemForm(i) => i.update(action),
            FormTui::ReceiptForm(r) => r.update(action),
        }
    }
}

impl FormTui {
    pub fn submit(&mut self) {
        match self {
            Self::UserFrom(u) => {
                if let Err(e) = u.submit() {
                    u.error = Some(e.to_string())
                }
            }
            Self::ItemForm(i) => {
                if let Err(e) = i.submit() {
                    i.error = Some(e.to_string())
                }
            }
            Self::ReceiptForm(r) => {
                if let Err(e) = r.submit() {
                    r.error = Some(e.to_string())
                }
            }
        }
    }
}
