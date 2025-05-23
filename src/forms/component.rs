use super::*;

impl Component for Form<'_> {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        self.render_block(rect, frame.buffer_mut());
    }

    fn update(&mut self, _action: Option<Action>) {
        todo!()
    }
}

impl Component for FormField<'_> {
    fn init(&mut self) {
        todo!()
    }

    fn draw(&mut self, _frame: &mut Frame, _rect: Rect) {
        todo!()
    }

    fn update(&mut self, _action: Option<Action>) {
        todo!()
    }
}

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
