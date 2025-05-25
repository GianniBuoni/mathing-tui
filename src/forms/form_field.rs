use super::*;

impl Component for FormField<'_> {
    fn init(&mut self, index: usize, tracker: Rc<RefCell<usize>>) {
        self.index = index;
        self.active_field = tracker;
        self.check_active();
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let style = Into::<AppStyles>::into(AppColors::get(self.active));

        let block = self.render_block(style.block_style);
        let input = self.render_input(style.input_style);

        block.render_ref(rect, frame.buffer_mut());
        input.render_ref(block.inner(rect), frame.buffer_mut());
    }

    fn update(&mut self, action: Option<Action>) {
        match action {
            Some(Action::SelectForward) | Some(Action::SelectBackward) => {
                self.check_active();
            }
            Some(_) => {}
            None => {}
        }
    }
}

impl FormField<'_> {
    fn assign_index(&mut self, index: usize) {
        self.index = index
    }

    fn check_active(&mut self) {
        if self.index == *self.active_field.borrow() {
            self.active = true
        }
    }
}
