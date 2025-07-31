use super::*;

impl Component for Dialogue {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        // get centered area
        let centered_rect = center_widget(
            rect,
            Constraint::Length(self.rect.width),
            Constraint::Length(self.rect.height),
        );
        // render clear in centered area
        frame.render_widget(Clear, centered_rect);

        // render message in block
        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::proportional(1));
        let inner_block = block.inner(centered_rect);

        block.render(centered_rect, frame.buffer_mut());
        self.message.render(inner_block, frame.buffer_mut());
    }
    fn handle_action(&mut self, action: Option<Action>) {
        let _ = action;
        todo!()
    }
}
