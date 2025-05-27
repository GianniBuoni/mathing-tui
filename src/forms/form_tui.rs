use super::*;

impl Component for FormTui<'_> {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        match self {
            FormTui::ItemForm(i) => i.draw(frame, rect),
            FormTui::ReceiptForm(r) => r.draw(frame, rect),
        }
    }

    fn update(&mut self, action: Option<Action>) {
        match self {
            FormTui::ItemForm(i) => i.update(action),
            FormTui::ReceiptForm(r) => r.update(action),
        }
    }
}

impl<'a> FormTui<'a> {
    pub fn submit(&self) -> Result<()> {
        match self {
            Self::ItemForm(i) => i.submit(),
            Self::ReceiptForm(r) => r.submit(),
        }
    }
}
